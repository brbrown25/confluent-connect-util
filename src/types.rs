use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub name: String,
    pub connector_class: String,
    pub config: HashMap<String, String>,
    pub sensitive_config: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub id: String,
    pub cluster_id: String,
    pub schema_registry_cluster_id: String,
}

// CLI and Application Types
#[derive(Debug, Default)]
pub struct ConnectorOptions {
    pub name: Option<String>,
    pub output: Option<String>,
}

// Terraform Types
#[derive(Debug)]
pub struct TerraformConfigOptions {
    pub connector_name: String,
    pub connector: ConnectorDefinition,
    pub topics: Vec<String>,
    pub input_data_format: Option<DataFormat>,
    pub output_data_format: Option<DataFormat>,
}

// Connector Definition Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorDefinition {
    pub name: String,
    pub display_name: String,
    pub connector_class: String,
    pub connector_type: ConnectorType,
    pub description: String,
    pub required_configs: Vec<ConfigField>,
    pub optional_configs: Vec<ConfigField>,
    pub sensitive_configs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectorType {
    Source,
    Sink,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataFormat {
    Avro,
    Json,
    JsonSr,
    Protobuf,
    Parquet,
}

impl DataFormat {
    pub fn to_terraform_value(&self) -> &'static str {
        match self {
            DataFormat::Avro => "AVRO",
            DataFormat::Json => "JSON",
            DataFormat::JsonSr => "JSON_SR",
            DataFormat::Protobuf => "PROTOBUF",
            DataFormat::Parquet => "PARQUET",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub field_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub valid_values: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_connector_config_creation() {
        let mut config = HashMap::new();
        config.insert(
            "connector.class".to_string(),
            "JdbcSourceConnector".to_string(),
        );
        config.insert(
            "database.url".to_string(),
            "jdbc:postgresql://localhost:5432/test".to_string(),
        );

        let mut sensitive_config = HashMap::new();
        sensitive_config.insert("database.password".to_string(), "secret".to_string());

        let connector_config = ConnectorConfig {
            name: "test-connector".to_string(),
            connector_class: "JdbcSourceConnector".to_string(),
            config,
            sensitive_config,
        };

        assert_eq!(connector_config.name, "test-connector");
        assert_eq!(connector_config.connector_class, "JdbcSourceConnector");
        assert_eq!(connector_config.config.len(), 2);
        assert_eq!(connector_config.sensitive_config.len(), 1);
    }

    #[test]
    fn test_environment_creation() {
        let environment = Environment {
            name: "test-env".to_string(),
            id: "env-123".to_string(),
            cluster_id: "cluster-456".to_string(),
            schema_registry_cluster_id: "sr-789".to_string(),
        };

        assert_eq!(environment.name, "test-env");
        assert_eq!(environment.id, "env-123");
        assert_eq!(environment.cluster_id, "cluster-456");
        assert_eq!(environment.schema_registry_cluster_id, "sr-789");
    }

    #[test]
    fn test_connector_config_serialize_deserialize() {
        let mut config = HashMap::new();
        config.insert(
            "connector.class".to_string(),
            "JdbcSourceConnector".to_string(),
        );

        let mut sensitive_config = HashMap::new();
        sensitive_config.insert("database.password".to_string(), "secret".to_string());

        let connector_config = ConnectorConfig {
            name: "test-connector".to_string(),
            connector_class: "JdbcSourceConnector".to_string(),
            config,
            sensitive_config,
        };

        // Test serialization
        let json = serde_json::to_string(&connector_config).unwrap();
        assert!(json.contains("test-connector"));
        assert!(json.contains("JdbcSourceConnector"));

        // Test deserialization
        let deserialized: ConnectorConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(connector_config.name, deserialized.name);
        assert_eq!(
            connector_config.connector_class,
            deserialized.connector_class
        );
    }

    #[test]
    fn test_connector_options_creation() {
        let options = ConnectorOptions {
            name: Some("test-connector".to_string()),
            output: Some("output.tf".to_string()),
        };

        assert_eq!(options.name, Some("test-connector".to_string()));
        assert_eq!(options.output, Some("output.tf".to_string()));
    }

    #[test]
    fn test_connector_options_default() {
        let options = ConnectorOptions::default();
        assert_eq!(options.name, None);
        assert_eq!(options.output, None);
    }

    #[test]
    fn test_connector_type_creation() {
        let source_type = ConnectorType::Source;
        let sink_type = ConnectorType::Sink;

        assert_eq!(source_type, ConnectorType::Source);
        assert_eq!(sink_type, ConnectorType::Sink);
        assert_ne!(source_type, sink_type);
    }

    #[test]
    fn test_data_format_enum() {
        let avro = DataFormat::Avro;
        let json = DataFormat::Json;
        let json_sr = DataFormat::JsonSr;
        let protobuf = DataFormat::Protobuf;
        let parquet = DataFormat::Parquet;

        assert_eq!(avro.to_terraform_value(), "AVRO");
        assert_eq!(json.to_terraform_value(), "JSON");
        assert_eq!(json_sr.to_terraform_value(), "JSON_SR");
        assert_eq!(protobuf.to_terraform_value(), "PROTOBUF");
        assert_eq!(parquet.to_terraform_value(), "PARQUET");
    }

    #[test]
    fn test_config_field_creation() {
        let config_field = ConfigField {
            name: "database.url".to_string(),
            display_name: "Database URL".to_string(),
            description: "JDBC URL for the database".to_string(),
            field_type: "STRING".to_string(),
            required: true,
            default_value: None,
            valid_values: None,
        };

        assert_eq!(config_field.name, "database.url");
        assert_eq!(config_field.display_name, "Database URL");
        assert_eq!(config_field.description, "JDBC URL for the database");
        assert_eq!(config_field.field_type, "STRING");
        assert!(config_field.required);
        assert_eq!(config_field.default_value, None);
        assert_eq!(config_field.valid_values, None);
    }

    #[test]
    fn test_connector_definition_creation() {
        let config_field = ConfigField {
            name: "database.url".to_string(),
            display_name: "Database URL".to_string(),
            description: "JDBC URL for the database".to_string(),
            field_type: "STRING".to_string(),
            required: true,
            default_value: None,
            valid_values: None,
        };

        let connector_def = ConnectorDefinition {
            name: "PostgresSink".to_string(),
            display_name: "PostgreSQL Sink".to_string(),
            connector_class: "io.confluent.connect.jdbc.JdbcSinkConnector".to_string(),
            connector_type: ConnectorType::Sink,
            description: "PostgreSQL Sink Connector".to_string(),
            required_configs: vec![config_field.clone()],
            optional_configs: vec![],
            sensitive_configs: vec!["password".to_string()],
        };

        assert_eq!(connector_def.name, "PostgresSink");
        assert_eq!(connector_def.display_name, "PostgreSQL Sink");
        assert_eq!(
            connector_def.connector_class,
            "io.confluent.connect.jdbc.JdbcSinkConnector"
        );
        assert_eq!(connector_def.connector_type, ConnectorType::Sink);
        assert_eq!(connector_def.description, "PostgreSQL Sink Connector");
        assert_eq!(connector_def.required_configs.len(), 1);
        assert_eq!(connector_def.optional_configs.len(), 0);
        assert_eq!(connector_def.sensitive_configs.len(), 1);
    }

    #[test]
    fn test_terraform_config_options_creation() {
        let config_field = ConfigField {
            name: "database.url".to_string(),
            display_name: "Database URL".to_string(),
            description: "JDBC URL for the database".to_string(),
            field_type: "STRING".to_string(),
            required: true,
            default_value: None,
            valid_values: None,
        };

        let connector_def = ConnectorDefinition {
            name: "PostgresSink".to_string(),
            display_name: "PostgreSQL Sink".to_string(),
            connector_class: "io.confluent.connect.jdbc.JdbcSinkConnector".to_string(),
            connector_type: ConnectorType::Sink,
            description: "PostgreSQL Sink Connector".to_string(),
            required_configs: vec![config_field],
            optional_configs: vec![],
            sensitive_configs: vec!["password".to_string()],
        };

        let terraform_options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector_def,
            topics: vec!["test-topic".to_string()],
            input_data_format: None,
            output_data_format: None,
        };

        assert_eq!(terraform_options.connector_name, "test-connector");
        assert_eq!(terraform_options.topics.len(), 1);
    }
}
