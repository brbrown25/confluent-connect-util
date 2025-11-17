use crate::error::ConnectUtilError;
use crate::types::{ConnectorDefinition, ConnectorType, DataFormat, TerraformConfigOptions};
use hcl::{Block, Body, Expression, Identifier, Object, ObjectKey, Traversal, Variable};

/// Terraform generator for creating connector configurations
pub struct TerraformGenerator;

impl TerraformGenerator {
    /// Generate a complete Terraform configuration for a connector
    pub fn generate_connector_config(
        &self,
        options: TerraformConfigOptions,
    ) -> Result<String, ConnectUtilError> {
        let resource_name = options.connector_name.replace('-', "_");

        // Build config_sensitive map as Expression::Object
        let mut config_sensitive_obj = Object::new();
        for sensitive_config in &options.connector.sensitive_configs {
            // Use Expression::String for keys with dots or special characters
            let key = if sensitive_config.contains('.') {
                ObjectKey::Expression(Expression::String(sensitive_config.clone()))
            } else {
                ObjectKey::Identifier(Identifier::new(sensitive_config.clone()).map_err(|e| {
                    ConnectUtilError::Terraform(format!(
                        "Invalid identifier '{}': {}",
                        sensitive_config, e
                    ))
                })?)
            };
            config_sensitive_obj.insert(
                key,
                Expression::String("<REPLACE_WITH_ACTUAL_VALUE>".to_string()),
            );
        }

        // Build config_nonsensitive map as Expression::Object
        let mut config_nonsensitive_obj = Object::new();
        config_nonsensitive_obj.insert(
            Self::make_object_key("connector.class"),
            Expression::String(options.connector.connector_class.clone()),
        );
        config_nonsensitive_obj.insert(
            Self::make_object_key("name"),
            Expression::String(options.connector_name.clone()),
        );
        config_nonsensitive_obj.insert(
            Self::make_object_key("kafka.auth.mode"),
            Expression::String("SERVICE_ACCOUNT".to_string()),
        );
        config_nonsensitive_obj.insert(
            Self::make_object_key("kafka.deployment.type"),
            Expression::String("DEDICATED".to_string()),
        );

        // Add topics configuration - handle connector-specific patterns
        if options.topics.is_empty() {
            if options.connector.connector_type == ConnectorType::Sink {
                config_nonsensitive_obj.insert(
                    Self::make_object_key("topics"),
                    Expression::String("<REPLACE_WITH_TOPIC_NAME>".to_string()),
                );
            } else {
                config_nonsensitive_obj.insert(
                    Self::make_object_key("topic.prefix"),
                    Expression::String("<REPLACE_WITH_TOPIC_PREFIX>".to_string()),
                );
            }
        } else {
            // Check if this connector uses topic.prefix (like PostgreSQL CDC Source V2)
            if options
                .connector
                .required_configs
                .iter()
                .any(|config| config.name == "topic.prefix")
            {
                config_nonsensitive_obj.insert(
                    Self::make_object_key("topic.prefix"),
                    Expression::String("<REPLACE_WITH_TOPIC_PREFIX>".to_string()),
                );
            }

            // Add topics field with join() to flatten array as comma-separated string
            let topic_values: Vec<Expression> = options
                .topics
                .iter()
                .map(|t| Expression::String(t.clone()))
                .collect();
            let join_expr = Expression::FuncCall(Box::new(hcl::FuncCall {
                name: Identifier::new("join").map_err(|e| {
                    ConnectUtilError::Terraform(format!("Invalid function name 'join': {}", e))
                })?,
                args: vec![
                    Expression::String(",".to_string()),
                    Expression::Array(topic_values),
                ],
                expand_final: false,
            }));
            config_nonsensitive_obj.insert(Self::make_object_key("topics"), join_expr);
        }

        // Add connector-specific configurations
        Self::add_connector_specific_config_to_object(
            &mut config_nonsensitive_obj,
            &options.connector,
            &options,
        )?;

        // Add output data format
        let output_format = options
            .output_data_format
            .as_ref()
            .unwrap_or(&DataFormat::Avro);
        let output_format_expr = Self::data_format_to_expression(output_format);
        config_nonsensitive_obj.insert(
            Self::make_object_key("output.data.format"),
            output_format_expr,
        );
        config_nonsensitive_obj.insert(
            Self::make_object_key("tasks.max"),
            Expression::String("1".to_string()),
        );

        // Build the resource block
        let resource_block = Block::builder("resource")
            .add_label("confluent_connector")
            .add_label(resource_name)
            .add_attribute((
                "status",
                Traversal::builder(Variable::new("var").map_err(|e| {
                    ConnectUtilError::Terraform(format!("Invalid variable name 'var': {}", e))
                })?)
                .attr("status")
                .build(),
            ))
            .add_block(
                Block::builder("environment")
                    .add_attribute((
                        "id",
                        Traversal::builder(Variable::new("var").map_err(|e| {
                            ConnectUtilError::Terraform(format!(
                                "Invalid variable name 'var': {}",
                                e
                            ))
                        })?)
                        .attr("environment_id")
                        .build(),
                    ))
                    .build(),
            )
            .add_block(
                Block::builder("kafka_cluster")
                    .add_attribute((
                        "id",
                        Traversal::builder(Variable::new("var").map_err(|e| {
                            ConnectUtilError::Terraform(format!(
                                "Invalid variable name 'var': {}",
                                e
                            ))
                        })?)
                        .attr("kafka_cluster")
                        .attr("id")
                        .build(),
                    ))
                    .build(),
            )
            .add_attribute(("config_sensitive", Expression::Object(config_sensitive_obj)))
            .add_attribute((
                "config_nonsensitive",
                Expression::Object(config_nonsensitive_obj),
            ))
            .add_block(
                Block::builder("lifecycle")
                    .add_attribute((
                        "ignore_changes",
                        Expression::Array(vec![
                            Expression::String(
                                "config_nonsensitive[\"kafka.deployment.type\"]".to_string(),
                            ),
                            Expression::String(
                                "config_nonsensitive[\"kafka.max.partition.validation.disable\"]"
                                    .to_string(),
                            ),
                            Expression::String(
                                "config_nonsensitive[\"kafka.max.partition.validation.enable\"]"
                                    .to_string(),
                            ),
                            Expression::String(
                                "config_nonsensitive[\"kafka.max.partition.validation\"]"
                                    .to_string(),
                            ),
                        ]),
                    ))
                    .build(),
            )
            .build();

        // Build the main body
        let body = Body::builder().add_block(resource_block).build();

        // Serialize to HCL string
        let hcl_string = hcl::to_string(&body)
            .map_err(|e| ConnectUtilError::Terraform(format!("Failed to serialize HCL: {}", e)))?;

        Ok(hcl_string)
    }

    /// Convert DataFormat to Expression for use in HCL
    fn data_format_to_expression(format: &DataFormat) -> Expression {
        // DataFormat::to_terraform_value() returns a string like "local.schema_formats.avro"
        // We need to convert this to a Traversal expression
        let terraform_value = format.to_terraform_value();
        // Parse the terraform value (e.g., "local.schema_formats.avro" -> Traversal)
        let parts: Vec<&str> = terraform_value.split('.').collect();
        if parts.len() >= 2 {
            if let Ok(var) = Variable::new(parts[0]) {
                let mut traversal = Traversal::builder(var);
                for part in parts.iter().skip(1) {
                    traversal = traversal.attr(*part);
                }
                Expression::Traversal(Box::new(traversal.build()))
            } else {
                // Fallback to string if parsing fails
                Expression::String(terraform_value.to_string())
            }
        } else {
            Expression::String(terraform_value.to_string())
        }
    }

    /// Helper to create ObjectKey from string
    pub(crate) fn make_object_key(s: &str) -> ObjectKey {
        if s.contains('.') {
            ObjectKey::Expression(Expression::String(s.to_string()))
        } else {
            ObjectKey::Identifier(Identifier::new(s).unwrap_or_else(|_| {
                // Fallback to Expression if identifier creation fails
                Identifier::sanitized(s)
            }))
        }
    }

    /// Add connector-specific configuration to the config object
    pub(crate) fn add_connector_specific_config_to_object(
        config_obj: &mut Object<ObjectKey, Expression>,
        connector_def: &ConnectorDefinition,
        options: &TerraformConfigOptions,
    ) -> Result<(), ConnectUtilError> {
        match connector_def.name.as_str() {
            "PostgresCdcSourceV2" => {
                config_obj.insert(
                    Self::make_object_key("database.hostname"),
                    Expression::String("<REPLACE_WITH_DATABASE_HOST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("database.port"),
                    Expression::String("5432".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("database.user"),
                    Expression::String("<REPLACE_WITH_DATABASE_USER>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("database.dbname"),
                    Expression::String("<REPLACE_WITH_DATABASE_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("database.sslmode"),
                    Expression::String("require".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("publication.name"),
                    Expression::String("dbz_publication".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("publication.autocreate.mode"),
                    Expression::String("filtered".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("snapshot.mode"),
                    Expression::String("initial".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("tombstones.on.delete"),
                    Expression::String("true".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("plugin.name"),
                    Expression::String("pgoutput".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("slot.name"),
                    Expression::String("dbz".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("poll.interval.ms"),
                    Expression::String("1000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("max.batch.size"),
                    Expression::String("1000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("event.processing.failure.handling.mode"),
                    Expression::String("fail".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("heartbeat.interval.ms"),
                    Expression::String("0".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("provide.transaction.metadata"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("decimal.handling.mode"),
                    Expression::String("precise".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("binary.handling.mode"),
                    Expression::String("bytes".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("time.precision.mode"),
                    Expression::String("adaptive".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("cleanup.policy"),
                    Expression::String("delete".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("hstore.handling.mode"),
                    Expression::String("json".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("interval.handling.mode"),
                    Expression::String("numeric".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("schema.refresh.mode"),
                    Expression::String("columns_diff".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("after.state.only"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("output.key.format"),
                    Self::data_format_to_expression(&DataFormat::Avro),
                );
            }
            "MySqlCdcSourceV2" => {
                config_obj.insert(
                    Self::make_object_key("database.server.name"),
                    Expression::String("<REPLACE_WITH_SERVER_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("database.ssl.mode"),
                    Expression::String("preferred".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("snapshot.mode"),
                    Expression::String("initial".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("binlog.buffer.size"),
                    Expression::String("8192".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("max.batch.size"),
                    Expression::String("2048".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("max.queue.size"),
                    Expression::String("8192".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("poll.interval.ms"),
                    Expression::String("1000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connect.timeout.ms"),
                    Expression::String("30000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("socket.timeout.ms"),
                    Expression::String("30000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("heartbeat.interval.ms"),
                    Expression::String("0".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("provide.transaction.metadata"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("decimal.handling.mode"),
                    Expression::String("precise".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("bigint.unsigned.handling.mode"),
                    Expression::String("long".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("binary.handling.mode"),
                    Expression::String("bytes".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("time.precision.mode"),
                    Expression::String("adaptive".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("cleanup.policy"),
                    Expression::String("delete".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("schema.refresh.mode"),
                    Expression::String("columns_diff".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("after.state.only"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("output.key.format"),
                    Self::data_format_to_expression(&DataFormat::Avro),
                );
            }
            "S3_SINK" => {
                config_obj.insert(
                    Self::make_object_key("s3.bucket.name"),
                    Expression::String("<REPLACE_WITH_BUCKET_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("s3.wan.mode"),
                    Expression::String("false".to_string()),
                );
                let input_format = options
                    .input_data_format
                    .as_ref()
                    .unwrap_or(&DataFormat::Avro);
                config_obj.insert(
                    Self::make_object_key("input.data.format"),
                    Self::data_format_to_expression(input_format),
                );
                let output_format = options
                    .output_data_format
                    .as_ref()
                    .unwrap_or(&DataFormat::Parquet);
                config_obj.insert(
                    Self::make_object_key("output.data.format"),
                    Self::data_format_to_expression(output_format),
                );
                config_obj.insert(
                    Self::make_object_key("topics.dir"),
                    Expression::String("<REPLACE_WITH_TOPICS_DIR>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("path.format"),
                    Expression::String("'effective_date'=YYYY-MM-dd".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("time.interval"),
                    Expression::String("HOURLY".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("rotate.schedule.interval.ms"),
                    Expression::String("3600000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("rotate.interval.ms"),
                    Expression::String("3600000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("flush.size"),
                    Expression::String("100000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("compression.codec"),
                    Expression::String("PARQUET - gzip".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("s3.compression.level"),
                    Expression::String("6".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("s3.part.size"),
                    Expression::String("5242880".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("kafka.max.partition.validation.disable"),
                    Expression::String("false".to_string()),
                );
            }
            "PostgreSQLSource" => {
                config_obj.insert(
                    Self::make_object_key("connection.host"),
                    Expression::String("<REPLACE_WITH_DB_HOST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.port"),
                    Expression::String("5432".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.user"),
                    Expression::String("<REPLACE_WITH_DB_USER>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.name"),
                    Expression::String("<REPLACE_WITH_DB_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("ssl.mode"),
                    Expression::String("prefer".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.whitelist"),
                    Expression::String("<REPLACE_WITH_TABLE_LIST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("mode"),
                    Expression::String("timestamp".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("poll.interval.ms"),
                    Expression::String("5000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.timezone"),
                    Expression::String("UTC".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.types"),
                    Expression::String("TABLE".to_string()),
                );
            }
            "PostgresSink" => {
                config_obj.insert(
                    Self::make_object_key("connection.host"),
                    Expression::String("<REPLACE_WITH_DB_HOST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.port"),
                    Expression::String("5432".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.user"),
                    Expression::String("<REPLACE_WITH_DB_USER>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.name"),
                    Expression::String("<REPLACE_WITH_DB_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("ssl.mode"),
                    Expression::String("prefer".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("insert.mode"),
                    Expression::String("UPSERT".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.name.format"),
                    Expression::String("<REPLACE_WITH_TABLE_FORMAT>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.types"),
                    Expression::String("TABLE".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.timezone"),
                    Expression::String("UTC".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("pk.mode"),
                    Expression::String("record_value".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("pk.fields"),
                    Expression::String("<REPLACE_WITH_PK_FIELDS>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("auto.create"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("auto.evolve"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("batch.sizes"),
                    Expression::String("5000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("max.poll.records"),
                    Expression::String("2500".to_string()),
                );
            }
            "MySQLSource" => {
                config_obj.insert(
                    Self::make_object_key("connection.host"),
                    Expression::String("<REPLACE_WITH_DB_HOST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.port"),
                    Expression::String("3306".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.user"),
                    Expression::String("<REPLACE_WITH_DB_USER>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.name"),
                    Expression::String("<REPLACE_WITH_DB_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("ssl.mode"),
                    Expression::String("preferred".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.whitelist"),
                    Expression::String("<REPLACE_WITH_TABLE_LIST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("mode"),
                    Expression::String("timestamp".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("poll.interval.ms"),
                    Expression::String("5000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.timezone"),
                    Expression::String("UTC".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.types"),
                    Expression::String("TABLE".to_string()),
                );
            }
            "MySQLSink" => {
                config_obj.insert(
                    Self::make_object_key("connection.host"),
                    Expression::String("<REPLACE_WITH_DB_HOST>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.port"),
                    Expression::String("3306".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("connection.user"),
                    Expression::String("<REPLACE_WITH_DB_USER>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.name"),
                    Expression::String("<REPLACE_WITH_DB_NAME>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("ssl.mode"),
                    Expression::String("preferred".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("insert.mode"),
                    Expression::String("UPSERT".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.name.format"),
                    Expression::String("<REPLACE_WITH_TABLE_FORMAT>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("table.types"),
                    Expression::String("TABLE".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("db.timezone"),
                    Expression::String("UTC".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("pk.mode"),
                    Expression::String("record_value".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("pk.fields"),
                    Expression::String("<REPLACE_WITH_PK_FIELDS>".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("auto.create"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("auto.evolve"),
                    Expression::String("false".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("batch.sizes"),
                    Expression::String("5000".to_string()),
                );
                config_obj.insert(
                    Self::make_object_key("max.poll.records"),
                    Expression::String("2500".to_string()),
                );
            }
            _ => {
                // Generic configuration for unknown connectors - no-op
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ConnectorDefinition, ConnectorType};

    fn create_test_connector() -> ConnectorDefinition {
        ConnectorDefinition {
            name: "PostgresSink".to_string(),
            display_name: "PostgreSQL Sink".to_string(),
            description: "PostgreSQL Sink Connector".to_string(),
            connector_class: "io.confluent.connect.jdbc.JdbcSinkConnector".to_string(),
            connector_type: ConnectorType::Sink,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec!["password".to_string()],
        }
    }

    #[test]
    fn test_generate_connector_config() {
        let generator = TerraformGenerator;
        let connector = create_test_connector();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector,
            topics: vec!["test-topic".to_string()],
            input_data_format: None,
            output_data_format: None,
        };

        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());

        let terraform = result.unwrap();
        assert!(terraform.contains("resource \"confluent_connector\" \"test_connector\""));
        assert!(terraform.contains("status = var.status"));
        assert!(terraform.contains("environment {"));
        assert!(terraform.contains("id = var.environment_id"));
        assert!(terraform.contains("kafka_cluster {"));
        assert!(terraform.contains("id = var.kafka_cluster.id"));
        assert!(terraform.contains("config_sensitive = {"));
        assert!(terraform.contains("config_nonsensitive = {"));
        assert!(terraform
            .contains("connector.class\" = \"io.confluent.connect.jdbc.JdbcSinkConnector\""));
        assert!(terraform.contains("lifecycle {"));
        assert!(terraform.contains("ignore_changes = ["));
    }

    #[test]
    fn test_generate_connector_config_production() {
        let generator = TerraformGenerator;
        let connector = create_test_connector();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector,
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };

        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());

        let terraform = result.unwrap();
        assert!(terraform.contains("resource \"confluent_connector\""));
        assert!(terraform.contains("status = var.status"));
        assert!(terraform.contains("environment {"));
        assert!(terraform.contains("kafka_cluster {"));
        assert!(terraform.contains("lifecycle {"));
    }

    #[test]
    fn test_add_connector_specific_config_postgres() {
        let mut config_obj = Object::new();
        let connector = ConnectorDefinition {
            name: "PostgresCdcSourceV2".to_string(),
            display_name: "PostgreSQL CDC Source V2".to_string(),
            description: "PostgreSQL CDC Source V2 Connector".to_string(),
            connector_class: "io.debezium.connector.postgresql.PostgresConnector".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector,
            &options,
        );
        assert!(result.is_ok());
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("database.hostname")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("database.port")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("plugin.name")));
    }

    #[test]
    fn test_add_connector_specific_config_mysql() {
        let mut config_obj = Object::new();
        let connector = ConnectorDefinition {
            name: "MySqlCdcSourceV2".to_string(),
            display_name: "MySQL CDC Source V2".to_string(),
            description: "MySQL CDC Source V2 Connector".to_string(),
            connector_class: "io.debezium.connector.mysql.MySqlConnector".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector,
            &options,
        );
        assert!(result.is_ok());
        assert!(
            config_obj.contains_key(&TerraformGenerator::make_object_key("database.server.name"))
        );
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("snapshot.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("binlog.buffer.size")));
    }

    #[test]
    fn test_add_connector_specific_config_s3() {
        let mut config_obj = Object::new();
        let connector = ConnectorDefinition {
            name: "S3_SINK".to_string(),
            display_name: "S3 Sink".to_string(),
            description: "S3 Sink Connector".to_string(),
            connector_class: "io.confluent.connect.s3.S3SinkConnector".to_string(),
            connector_type: ConnectorType::Sink,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector,
            &options,
        );
        assert!(result.is_ok());
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("s3.bucket.name")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("s3.wan.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("input.data.format")));
    }

    #[test]
    fn test_add_connector_specific_config_unknown() {
        let mut config_obj = Object::new();
        let connector = ConnectorDefinition {
            name: "UnknownConnector".to_string(),
            display_name: "Unknown Connector".to_string(),
            description: "Unknown Connector".to_string(),
            connector_class: "com.example.UnknownConnector".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector,
            &options,
        );
        assert!(result.is_ok());
        // Unknown connectors should not add any config
        assert!(config_obj.is_empty());
    }
}
