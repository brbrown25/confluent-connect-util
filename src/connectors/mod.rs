use crate::types::{ConfigField, ConnectorDefinition, ConnectorType};
use std::collections::HashMap;

mod sinks;
mod sources;

// Re-export connector functions for use in get_all_connectors
use sinks::*;
use sources::*;

// Helper function to create ConfigField with common defaults
// This is used by both sources and sinks modules
pub(crate) fn config_field(
    name: &str,
    description: &str,
    field_type: &str,
    required: bool,
    valid_values: Option<Vec<String>>,
) -> ConfigField {
    ConfigField {
        name: name.to_string(),
        display_name: name.to_string(),
        description: description.to_string(),
        field_type: field_type.to_string(),
        required,
        default_value: None,
        valid_values,
    }
}

impl ConnectorDefinition {
    pub fn get_all_connectors() -> Vec<ConnectorDefinition> {
        vec![
            // Source Connectors
            activemq_source(),
            amazon_cloudwatch_logs_source(),
            amazon_dynamodb_cdc_source(),
            amazon_kinesis_source(),
            amazon_s3_source(),
            amazon_sqs_source(),
            azure_blob_storage_source(),
            azure_cosmos_db_source(),
            azure_cosmos_db_source_v2(),
            azure_event_hubs_source(),
            azure_service_bus_source(),
            couchbase_source(),
            datagen_source(),
            github_source(),
            google_cloud_pubsub_source(),
            http_source(),
            http_source_v2(),
            ibm_mq_source(),
            influxdb_2_source(),
            jira_source(),
            mariadb_cdc_source(),
            microsoft_sql_server_cdc_source_v2(),
            microsoft_sql_server_source(),
            mongodb_atlas_source(),
            mqtt_source(),
            mysql_cdc_source_v2(),
            mysql_cdc_source(),
            mysql_source(),
            oracle_cdc_source(),
            oracle_xstream_cdc_source(),
            oracle_database_source(),
            postgresql_cdc_source_v2(),
            postgresql_cdc_source(),
            postgresql_source(),
            rabbitmq_source(),
            salesforce_bulk_api_source(),
            salesforce_bulk_api_2_0_source(),
            salesforce_cdc_source(),
            salesforce_platform_event_source(),
            salesforce_pushtopic_source(),
            servicenow_source_v2(),
            sftp_source(),
            snowflake_source(),
            zendesk_source(),
            // Sink Connectors
            alloydb_sink(),
            amazon_s3_sink(),
            snowflake_sink(),
            postgresql_sink(),
            mysql_sink(),
            microsoft_sql_server_sink(),
            oracle_sink(),
            mongodb_sink(),
            elasticsearch_sink(),
            bigquery_sink(),
            redshift_sink(),
            databricks_sink(),
            jdbc_sink(),
            splunk_sink(),
            clickhouse_sink(),
        ]
    }

    pub fn get_connectors_by_type(connector_type: &ConnectorType) -> Vec<ConnectorDefinition> {
        Self::get_all_connectors()
            .into_iter()
            .filter(|connector| {
                std::mem::discriminant(&connector.connector_type)
                    == std::mem::discriminant(connector_type)
            })
            .collect()
    }

    pub fn get_connector_by_name(name: &str) -> Option<ConnectorDefinition> {
        Self::get_all_connectors()
            .into_iter()
            .find(|connector| connector.name == name)
    }

    pub fn validate_config(
        &self,
        config_nonsensitive: &HashMap<String, String>,
        config_sensitive: &HashMap<String, String>,
    ) -> Result<(), String> {
        // Check required configs (should be in either block)
        let mut all_config = config_nonsensitive.clone();
        all_config.extend(config_sensitive.clone());

        for required_config in &self.required_configs {
            if !all_config.contains_key(&required_config.name) {
                return Err(format!(
                    "Missing required configuration: {}",
                    required_config.name
                ));
            }
        }

        // Check sensitive configs are not in non-sensitive config (unless they're empty strings)
        for sensitive_config in &self.sensitive_configs {
            if let Some(value) = config_nonsensitive.get(sensitive_config) {
                if !value.is_empty() {
                    return Err(format!(
                        "Sensitive configuration '{}' should be in config_sensitive block",
                        sensitive_config
                    ));
                }
            }
        }

        // Validate field values
        for (key, value) in &all_config {
            if let Some(field) = self
                .required_configs
                .iter()
                .chain(self.optional_configs.iter())
                .find(|f| &f.name == key)
            {
                if let Some(valid_values) = &field.valid_values {
                    if !valid_values.contains(value) {
                        return Err(format!(
                            "Invalid value '{}' for field '{}'. Valid values: {:?}",
                            value, key, valid_values
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
