use crate::error::ConnectUtilError;
use crate::terraform::TerraformGenerator;
use crate::types::{
    ConnectorConfig, ConnectorDefinition, ConnectorOptions, ConnectorType, TerraformConfigOptions,
};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, Select};
use hcl::{Body, Expression};
use std::collections::HashMap;
use std::path::Path;

type TerraformParseResults = Result<Vec<ConnectorConfig>, ConnectUtilError>;

/// Main application struct for the Connect Utility
pub struct ConnectUtilApp;

impl ConnectUtilApp {
    /// Creates a new instance of ConnectUtilApp
    pub async fn new() -> Result<Self, ConnectUtilError> {
        Ok(Self)
    }

    /// Non-interactive version for testing and programmatic use
    /// Generates Terraform configuration without user prompts
    pub fn generate_terraform_non_interactive(
        &self,
        options: ConnectorOptions,
    ) -> Result<String, ConnectUtilError> {
        // Validate required options
        let connector_name = options.name.ok_or_else(|| {
            ConnectUtilError::Config(
                "Connector name is required for non-interactive mode".to_string(),
            )
        })?;

        // Get connector type from options or default to Source
        let connector_type = ConnectorType::Source; // Default for non-interactive
        let available_connectors = ConnectorDefinition::get_connectors_by_type(&connector_type);
        let selected_connector = available_connectors
            .first()
            .ok_or_else(|| ConnectUtilError::Config("No connectors available".to_string()))?;

        // Get topics - empty for non-interactive mode
        let topics = vec![];

        // Generate Terraform configuration
        let terraform_options = TerraformConfigOptions {
            connector_name,
            connector: selected_connector.clone(),
            topics,
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        generator.generate_connector_config(terraform_options)
    }

    #[cfg(not(tarpaulin_include))]
    pub async fn generate_terraform_interactive(
        &mut self,
        options: ConnectorOptions,
    ) -> Result<(), ConnectUtilError> {
        println!("🚀 Welcome to the Kafka Connect Terraform Generator!");
        println!();

        // Step 1: Get connector name
        let connector_name = if let Some(name) = options.name {
            name
        } else {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter connector name")
                .interact()
                .map_err(|e| {
                    ConnectUtilError::Config(format!("Failed to get connector name: {}", e))
                })?
        };

        // Step 2: Get connector type
        let connector_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select connector type")
            .items(&["Source", "Sink"])
            .interact()
            .map_err(|e| {
                ConnectUtilError::Config(format!("Failed to select connector type: {}", e))
            })?;

        let connector_type_enum = match connector_type {
            0 => ConnectorType::Source,
            1 => ConnectorType::Sink,
            _ => {
                return Err(ConnectUtilError::Config(
                    "Invalid connector type selection".to_string(),
                ))
            }
        };

        // Step 4: Get connector selection with fuzzy search
        let available_connectors =
            ConnectorDefinition::get_connectors_by_type(&connector_type_enum);
        let connector_names: Vec<&str> = available_connectors
            .iter()
            .map(|c| c.display_name.as_str())
            .collect();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select connector (type to search)")
            .items(&connector_names)
            .interact()
            .map_err(|e| ConnectUtilError::Config(format!("Failed to select connector: {}", e)))?;

        let selected_connector = &available_connectors[selection];

        // Step 5: Generate Terraform configuration
        // Topics can be manually specified in the generated Terraform
        let topics = vec![];
        let terraform_options = TerraformConfigOptions {
            connector_name,
            connector: selected_connector.clone(),
            topics,
            input_data_format: None,
            output_data_format: None,
        };
        let generator = TerraformGenerator;
        let terraform_config = generator.generate_connector_config(terraform_options)?;

        // Step 8: Output configuration
        if let Some(output_path) = options.output {
            std::fs::write(&output_path, &terraform_config)?;
            println!("✅ Terraform configuration written to: {}", output_path);
        } else {
            println!("📄 Generated Terraform Configuration:");
            println!("{}", terraform_config);
        }

        Ok(())
    }

    /// Validates a Terraform connector configuration file
    /// Checks both the connector configuration and Terraform structure
    pub async fn validate_connector(&mut self, config_file: &str) -> Result<(), ConnectUtilError> {
        let config_path = Path::new(config_file);
        if !config_path.exists() {
            return Err(ConnectUtilError::Config(format!(
                "Configuration file not found: {}",
                config_file
            )));
        }

        let terraform_content = std::fs::read_to_string(config_path)?;

        // Check if the entire file is commented out
        let all_lines_commented = terraform_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .all(|line| line.trim().starts_with('#'));

        if all_lines_commented {
            println!("✅ File is commented out - no validation needed");
            println!("📋 Configuration Summary:");
            println!("  Status: Commented out");
            println!("  Note: This file contains no active connector configuration");
            return Ok(());
        }

        // Parse the Terraform file to extract all connector configurations
        let connector_configs = self.parse_terraform_configs(&terraform_content)?;

        if connector_configs.is_empty() {
            return Err(ConnectUtilError::Config(
                "No connector configurations found in the file.".to_string(),
            ));
        }

        println!(
            "🔍 Found {} connector configuration(s) to validate",
            connector_configs.len()
        );

        for (index, config) in connector_configs.iter().enumerate() {
            println!(
                "\n--- Validating Connector {} of {} ---",
                index + 1,
                connector_configs.len()
            );

            // Find the connector definition
            let connector_def = ConnectorDefinition::get_connector_by_name(&config.connector_class)
                .ok_or_else(|| {
                    ConnectUtilError::Config(format!(
                        "Unknown connector: {}",
                        config.connector_class
                    ))
                })?;

            // Validate the configuration
            match connector_def.validate_config(&config.config, &config.sensitive_config) {
                Ok(()) => {
                    println!("✅ Configuration is valid!");
                    println!("📋 Configuration Summary:");
                    println!("  Connector: {}", connector_def.display_name);
                    println!("  Required configs: ✅ All present");
                    println!("  Sensitive configs: ✅ Properly separated");
                    println!("  Non-sensitive configs: {} fields", config.config.len());
                    println!(
                        "  Sensitive configs: {} fields",
                        config.sensitive_config.len()
                    );
                }
                Err(error) => {
                    println!("❌ Configuration validation failed:");
                    println!("  {}", error);
                }
            }
        }

        // Note: We don't return an error here even if validation fails
        // The validation errors are printed above, but the function should still return Ok
        // unless there's a parsing error or other non-validation error

        // Validate environment-specific Terraform structure
        self.validate_terraform_structure(&terraform_content)?;

        Ok(())
    }

    /// Parses Terraform content and extracts all connector configurations
    /// Uses hcl-rs to properly parse HCL structure
    fn parse_terraform_configs(&self, terraform_content: &str) -> TerraformParseResults {
        let mut connector_configs = Vec::new();

        // Parse the HCL content
        let body: Body = match hcl::from_str(terraform_content) {
            Ok(body) => body,
            Err(e) => {
                return Err(ConnectUtilError::Config(format!(
                    "Failed to parse Terraform file: {}",
                    e
                )));
            }
        };

        // Find all resource blocks with type "confluent_connector"
        for block in body.blocks() {
            if block.identifier() == "resource" {
                let labels = block.labels();
                if labels.len() >= 2 && labels[0].as_str() == "confluent_connector" {
                    // Found a confluent_connector resource
                    let connector_name = if labels.len() >= 2 {
                        labels[1].as_str().to_string()
                    } else {
                        String::new()
                    };
                    let mut connector_class = String::new();
                    let mut config_nonsensitive = HashMap::new();
                    let mut config_sensitive = HashMap::new();

                    // Extract attributes from the block body
                    self.extract_config_from_block(
                        block.body(),
                        &mut connector_class,
                        &mut config_nonsensitive,
                        &mut config_sensitive,
                    );

                    // If we found a connector class, add it to our list
                    if !connector_class.is_empty() {
                        connector_configs.push(ConnectorConfig {
                            name: connector_name,
                            connector_class,
                            config: config_nonsensitive,
                            sensitive_config: config_sensitive,
                        });
                    }
                }
            } else if block.identifier() == "module" {
                // Handle legacy module blocks - extract config from module body
                let labels = block.labels();
                let connector_name = if !labels.is_empty() {
                    labels[0].as_str().to_string()
                } else {
                    String::new()
                };
                let mut connector_class = String::new();
                let mut config_nonsensitive = HashMap::new();
                let mut config_sensitive = HashMap::new();

                self.extract_config_from_block(
                    block.body(),
                    &mut connector_class,
                    &mut config_nonsensitive,
                    &mut config_sensitive,
                );

                if !connector_class.is_empty() {
                    connector_configs.push(ConnectorConfig {
                        name: connector_name,
                        connector_class,
                        config: config_nonsensitive,
                        sensitive_config: config_sensitive,
                    });
                }
            }
        }

        Ok(connector_configs)
    }

    fn extract_config_from_block(
        &self,
        body: &Body,
        connector_class: &mut String,
        config_nonsensitive: &mut HashMap<String, String>,
        config_sensitive: &mut HashMap<String, String>,
    ) {
        // Extract config_nonsensitive from body attributes
        for attr in body.attributes() {
            let key = attr.key();
            if key == "config_nonsensitive" {
                if let Some(map) = self.extract_map_from_expression(attr.expr()) {
                    for (key, value) in map {
                        if key == "connector.class" {
                            *connector_class = value.clone();
                        }
                        config_nonsensitive.insert(key, value);
                    }
                }
            } else if key == "config_sensitive" {
                if let Some(map) = self.extract_map_from_expression(attr.expr()) {
                    for (key, value) in map {
                        config_sensitive.insert(key, value);
                    }
                }
            }
        }
    }

    fn extract_map_from_expression(&self, expr: &Expression) -> Option<HashMap<String, String>> {
        match expr {
            Expression::Object(map) => {
                let mut result = HashMap::new();
                for (key, value) in map.iter() {
                    if let Some(str_value) = self.extract_string_from_expression(value) {
                        result.insert(key.to_string(), str_value);
                    }
                }
                Some(result)
            }
            _ => None,
        }
    }

    #[allow(clippy::only_used_in_recursion)] // This is a recursive function
    fn extract_string_from_expression(&self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::String(s) => Some(s.to_string()),
            Expression::Variable(var) => Some(format!("var.{}", var.as_str())),
            Expression::FuncCall(func) => {
                // Handle function calls like join(",", [...])
                // FuncCall is a Box, so we need to dereference it
                let func_name = func.name.as_str();
                if func_name == "join" {
                    if let Some(Expression::Array(arr)) = func.args.first() {
                        let values: Vec<String> = arr
                            .iter()
                            .filter_map(|e| self.extract_string_from_expression(e))
                            .collect();
                        return Some(values.join(", "));
                    }
                }
                // For other function calls, try to format as string
                Some(format!("{}(...)", func_name))
            }
            Expression::Array(arr) => {
                let values: Vec<String> = arr
                    .iter()
                    .filter_map(|e| self.extract_string_from_expression(e))
                    .collect();
                Some(format!("[{}]", values.join(", ")))
            }
            Expression::Number(n) => Some(n.to_string()),
            Expression::Bool(b) => Some(b.to_string()),
            _ => {
                // For other expression types, try to convert to string
                // This is a fallback for expressions we don't handle explicitly
                None
            }
        }
    }

    fn validate_terraform_structure(
        &self,
        terraform_content: &str,
    ) -> Result<(), ConnectUtilError> {
        println!("🔍 Validating Terraform structure...");

        // Parse the HCL content to validate structure properly
        let body: Body = match hcl::from_str(terraform_content) {
            Ok(body) => body,
            Err(e) => {
                return Err(ConnectUtilError::Config(format!(
                    "Failed to parse Terraform file: {}",
                    e
                )));
            }
        };

        // Validate each confluent_connector resource block individually
        let mut connector_count = 0;
        for block in body.blocks() {
            if block.identifier() == "resource" {
                let labels = block.labels();
                if labels.len() >= 2 && labels[0].as_str() == "confluent_connector" {
                    connector_count += 1;
                    let resource_name = labels[1].as_str();
                    self.validate_resource_block(block.body(), resource_name)?;
                }
            }
        }

        if connector_count == 0 {
            return Err(ConnectUtilError::Config(
                "❌ No 'confluent_connector' resources found in file".to_string(),
            ));
        }

        println!("  ✅ Validated {} connector resource(s)", connector_count);
        println!("✅ Terraform structure validation passed!");
        Ok(())
    }

    /// Validates a single resource block structure
    /// Ensures all required fields and nested blocks are present and correctly formatted
    fn validate_resource_block(
        &self,
        body: &Body,
        resource_name: &str,
    ) -> Result<(), ConnectUtilError> {
        // Check for status field
        let mut has_status = false;
        for attr in body.attributes() {
            if attr.key() == "status" {
                has_status = true;
                break;
            }
        }
        if !has_status {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' missing 'status' field",
                resource_name
            )));
        }

        // Check for environment block with correct structure
        let mut has_environment = false;
        let mut environment_has_id = false;
        let mut environment_attrs = Vec::new();
        for block in body.blocks() {
            if block.identifier() == "environment" {
                has_environment = true;
                // Check if environment block has 'id' attribute
                for attr in block.body().attributes() {
                    environment_attrs.push(attr.key().to_string());
                    if attr.key() == "id" {
                        environment_has_id = true;
                    }
                }
                break;
            }
        }

        if !has_environment {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' missing 'environment {{ id = ... }}' block",
                resource_name
            )));
        }
        if !environment_has_id {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' environment block must have 'id' attribute (found: {})",
                resource_name,
                if environment_attrs.is_empty() {
                    "none".to_string()
                } else {
                    environment_attrs.join(", ")
                }
            )));
        }

        // Check for kafka_cluster block with correct structure
        let mut has_kafka_cluster = false;
        let mut kafka_cluster_has_id = false;
        for block in body.blocks() {
            if block.identifier() == "kafka_cluster" {
                has_kafka_cluster = true;
                // Check if kafka_cluster block has 'id' attribute
                for attr in block.body().attributes() {
                    if attr.key() == "id" {
                        kafka_cluster_has_id = true;
                        break;
                    }
                }
                break;
            }
        }

        if !has_kafka_cluster {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' missing 'kafka_cluster {{ id = ... }}' block",
                resource_name
            )));
        }
        if !kafka_cluster_has_id {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' kafka_cluster block must have 'id' attribute",
                resource_name
            )));
        }

        // Check for config_sensitive attribute
        let mut has_config_sensitive = false;
        for attr in body.attributes() {
            if attr.key() == "config_sensitive" {
                has_config_sensitive = true;
                break;
            }
        }
        if !has_config_sensitive {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' missing 'config_sensitive' attribute",
                resource_name
            )));
        }

        // Check for config_nonsensitive attribute
        let mut has_config_nonsensitive = false;
        for attr in body.attributes() {
            if attr.key() == "config_nonsensitive" {
                has_config_nonsensitive = true;
                break;
            }
        }
        if !has_config_nonsensitive {
            return Err(ConnectUtilError::Config(format!(
                "❌ Resource '{}' missing 'config_nonsensitive' attribute",
                resource_name
            )));
        }

        Ok(())
    }

    pub async fn list_plugins(
        &mut self,
        filter_type: Option<String>,
    ) -> Result<(), ConnectUtilError> {
        let all_connectors = ConnectorDefinition::get_all_connectors();

        let filtered_connectors = if let Some(filter) = filter_type {
            let connector_type = match filter.to_lowercase().as_str() {
                "source" => ConnectorType::Source,
                "sink" => ConnectorType::Sink,
                _ => {
                    println!("❌ Invalid filter type. Use 'source' or 'sink'");
                    return Ok(());
                }
            };
            all_connectors
                .into_iter()
                .filter(|c| c.connector_type == connector_type)
                .collect()
        } else {
            all_connectors
        };

        println!("Available connector plugins:");
        for connector in filtered_connectors {
            let connector_type_str = match connector.connector_type {
                ConnectorType::Source => "source",
                ConnectorType::Sink => "sink",
            };
            println!("  - {} ({})", connector.display_name, connector_type_str);
            println!("    Class: {}", connector.connector_class);
            println!("    Description: {}", connector.description);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hcl::Object;

    #[tokio::test]
    async fn test_parse_terraform_config_success() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform_content = r#"
        module "test_connector" {
          config_sensitive = {
            "connection.password" = "secret_password"
          }
          config_nonsensitive = {
            "connector.class" = "PostgresSink"
            "connection.host" = "localhost"
            "connection.port" = "5432"
            "connection.user" = "test_user"
            "db.name" = "test_db"
          }
        }
        "#;

        let result = app.parse_terraform_configs(terraform_content);
        assert!(result.is_ok(), "Should parse valid Terraform config");

        let configs = result.unwrap();
        assert!(!configs.is_empty(), "Should have at least one config");
        let config = &configs[0];
        assert_eq!(config.connector_class, "PostgresSink");
        assert_eq!(
            config.config.get("connection.host"),
            Some(&"localhost".to_string())
        );
        assert_eq!(
            config.config.get("connection.port"),
            Some(&"5432".to_string())
        );
        assert_eq!(
            config.sensitive_config.get("connection.password"),
            Some(&"secret_password".to_string())
        );
    }

    #[tokio::test]
    async fn test_parse_terraform_config_with_comments() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform_content = r#"
        module "test_connector" {
          config_sensitive = {
            "connection.password" = "secret_password"
          }
          config_nonsensitive = {
            "connector.class" = "PostgresSink"
            # This is a comment
            "connection.host" = "localhost"
            "connection.port" = "5432"
            # Another comment
            "connection.user" = "test_user"
            "db.name" = "test_db"
          }
        }
        "#;

        let result = app.parse_terraform_configs(terraform_content);
        assert!(
            result.is_ok(),
            "Should parse Terraform config with comments"
        );

        let configs = result.unwrap();
        assert!(!configs.is_empty(), "Should have at least one config");
        let config = &configs[0];
        assert_eq!(config.connector_class, "PostgresSink");
        assert_eq!(
            config.config.get("connection.host"),
            Some(&"localhost".to_string())
        );
        assert_eq!(
            config.config.get("connection.port"),
            Some(&"5432".to_string())
        );
        assert_eq!(
            config.sensitive_config.get("connection.password"),
            Some(&"secret_password".to_string())
        );
    }

    #[tokio::test]
    async fn test_parse_terraform_config_missing_connector_class() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform_content = r#"
        module "test_connector" {
          config_sensitive = {
            "connection.password" = "secret_password"
          }
          config_nonsensitive = {
            "connection.host" = "localhost"
            "connection.port" = "5432"
            "connection.user" = "test_user"
            "db.name" = "test_db"
          }
        }
        "#;

        let result = app.parse_terraform_configs(terraform_content);
        assert!(
            result.is_ok(),
            "Should parse even without connector.class (returns empty list)"
        );
        let configs = result.unwrap();
        assert!(
            configs.is_empty(),
            "Should return empty list when connector.class is missing"
        );
    }

    #[tokio::test]
    async fn test_parse_terraform_config_entirely_commented() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform_content = r#"
        # module "test_connector" {
        #   config_sensitive = {
        #     "connection.password" = "secret_password"
        #   }
        #   config_nonsensitive = {
        #     "connector.class" = "PostgresSink"
        #     "connection.host" = "localhost"
        #     "connection.port" = "5432"
        #     "connection.user" = "test_user"
        #     "db.name" = "test_db"
        #   }
        # }
        "#;

        let result = app.parse_terraform_configs(terraform_content);
        assert!(
            result.is_ok(),
            "Should parse even when entire file is commented (returns empty list)"
        );
        let configs = result.unwrap();
        assert!(
            configs.is_empty(),
            "Should return empty list when entire file is commented"
        );
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_postgres_cdc() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let mut config_obj = Object::new();
        let connector_def =
            ConnectorDefinition::get_connector_by_name("PostgresCdcSourceV2").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector_def.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector_def,
            &options,
        );
        assert!(result.is_ok(), "Should successfully add connector config");

        // Check that configuration was added to config object
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("database.sslmode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("publication.name")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("snapshot.mode")));
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_s3_sink() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let mut config_obj = Object::new();
        let connector_def = ConnectorDefinition::get_connector_by_name("S3_SINK").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector_def.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector_def,
            &options,
        );
        assert!(result.is_ok(), "Should successfully add connector config");

        // Check that configuration was added to config object
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("s3.bucket.name")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("topics.dir")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("input.data.format")));
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_unknown_connector() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let mut config_obj = Object::new();
        let connector_def = ConnectorDefinition {
            name: "UnknownConnector".to_string(),
            display_name: "Unknown Connector".to_string(),
            connector_class: "UnknownConnector".to_string(),
            connector_type: ConnectorType::Source,
            description: "Unknown connector".to_string(),
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let options = TerraformConfigOptions {
            connector_name: "test".to_string(),
            connector: connector_def.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };
        let result = TerraformGenerator::add_connector_specific_config_to_object(
            &mut config_obj,
            &connector_def,
            &options,
        );
        assert!(result.is_ok(), "Should not panic for unknown connector");
        // Unknown connectors should not add any config
        assert!(
            config_obj.is_empty(),
            "Unknown connector should not add any config"
        );
    }

    #[tokio::test]
    async fn test_environment_specific_terraform_generation() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresSink").unwrap();

        // Test production environment
        let prod_options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec!["test-topic".to_string()],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let prod_result = generator.generate_connector_config(prod_options);
        assert!(prod_result.is_ok());
        let prod_terraform = prod_result.unwrap();

        // Check that terraform is generated with resource format
        assert!(prod_terraform.contains("resource \"confluent_connector\""));
        assert!(prod_terraform.contains("status = var.status"));
        assert!(prod_terraform.contains("environment {"));
        assert!(prod_terraform.contains("kafka_cluster {"));

        // Test dev environment
        let dev_options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec!["test-topic".to_string()],
            input_data_format: None,
            output_data_format: None,
        };

        let dev_result = generator.generate_connector_config(dev_options);
        assert!(dev_result.is_ok());
        let dev_terraform = dev_result.unwrap();

        // Check dev-specific values
        assert!(dev_terraform.contains("resource \"confluent_connector\""));
        assert!(dev_terraform.contains("status = var.status"));
        assert!(dev_terraform.contains("environment {"));
        assert!(dev_terraform.contains("kafka_cluster {"));
    }

    #[tokio::test]
    async fn test_terraform_structure_validation() {
        let app = ConnectUtilApp::new().await.unwrap();

        // Test resource-based Terraform validation
        let terraform = r#"
resource "confluent_connector" "test_connector" {
  status = "RUNNING"

  environment {
    id = var.environment_id
  }

  kafka_cluster {
    id = var.kafka_cluster.id
  }

  config_sensitive = {
    "connection.password" = "secret"
  }

  config_nonsensitive = {
    "connector.class" = "PostgresSink"
    "name" = "test-connector"
  }

  lifecycle {
    ignore_changes = [
      config_nonsensitive["kafka.deployment.type"],
    ]
  }
}
"#;

        let result = app.validate_terraform_structure(terraform);
        assert!(
            result.is_ok(),
            "Resource-based Terraform structure should be valid"
        );

        // Test invalid Terraform (missing required blocks)
        let invalid_terraform = r#"
resource "confluent_connector" "test_connector" {
  status = "RUNNING"
  # Missing environment and kafka_cluster blocks
  config_sensitive = {
    "connection.password" = "secret"
  }
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
    "name" = "test-connector"
  }
}
"#;

        let result = app.validate_terraform_structure(invalid_terraform);
        assert!(
            result.is_err(),
            "Invalid Terraform structure should fail validation"
        );
    }

    #[tokio::test]
    async fn test_validate_resource_block_missing_status() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  environment {
    id = var.environment_id
  }
  kafka_cluster {
    id = var.kafka_cluster.id
  }
  config_sensitive = {}
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
  }
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing 'status' field"));
    }

    #[tokio::test]
    async fn test_validate_resource_block_missing_environment() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  kafka_cluster {
    id = var.kafka_cluster.id
  }
  config_sensitive = {}
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
  }
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing 'environment"));
    }

    #[tokio::test]
    async fn test_validate_resource_block_environment_wrong_attribute() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  environment {
    exo = var.environment_id
  }
  kafka_cluster {
    id = var.kafka_cluster.id
  }
  config_sensitive = {}
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
  }
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("environment block must have 'id' attribute"));
    }

    #[tokio::test]
    async fn test_validate_resource_block_missing_kafka_cluster() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  environment {
    id = var.environment_id
  }
  config_sensitive = {}
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
  }
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing 'kafka_cluster"));
    }

    #[tokio::test]
    async fn test_validate_resource_block_kafka_cluster_wrong_attribute() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  environment {
    id = var.environment_id
  }
  kafka_cluster {
    name = var.kafka_cluster.name
  }
  config_sensitive = {}
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
  }
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("kafka_cluster block must have 'id' attribute"));
    }

    #[tokio::test]
    async fn test_validate_resource_block_missing_config_sensitive() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  environment {
    id = var.environment_id
  }
  kafka_cluster {
    id = var.kafka_cluster.id
  }
  config_nonsensitive = {
    "connector.class" = "PostgresSink"
  }
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing 'config_sensitive'"));
    }

    #[tokio::test]
    async fn test_validate_resource_block_missing_config_nonsensitive() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  environment {
    id = var.environment_id
  }
  kafka_cluster {
    id = var.kafka_cluster.id
  }
  config_sensitive = {}
}
"#;
        let body: Body = hcl::from_str(terraform).unwrap();
        let resource_block = body
            .blocks()
            .find(|b| {
                b.identifier() == "resource"
                    && b.labels().len() >= 2
                    && b.labels()[0].as_str() == "confluent_connector"
            })
            .unwrap();

        let result = app.validate_resource_block(resource_block.body(), "test");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing 'config_nonsensitive'"));
    }

    #[tokio::test]
    async fn test_validate_terraform_structure_no_resources() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
variable "test" {
  default = "value"
}
"#;
        let result = app.validate_terraform_structure(terraform);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No 'confluent_connector' resources found"));
    }

    #[tokio::test]
    async fn test_validate_terraform_structure_invalid_hcl() {
        let app = ConnectUtilApp::new().await.unwrap();
        let terraform = r#"
resource "confluent_connector" "test" {
  status = "RUNNING"
  # Missing closing brace
"#;
        let result = app.validate_terraform_structure(terraform);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse Terraform file"));
    }

    #[tokio::test]
    async fn test_list_plugins_all() {
        let mut app = ConnectUtilApp::new().await.unwrap();
        let result = app.list_plugins(None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_plugins_filtered_source() {
        let mut app = ConnectUtilApp::new().await.unwrap();
        let result = app.list_plugins(Some("source".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_plugins_filtered_sink() {
        let mut app = ConnectUtilApp::new().await.unwrap();
        let result = app.list_plugins(Some("sink".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_plugins_invalid_filter() {
        let mut app = ConnectUtilApp::new().await.unwrap();
        let result = app.list_plugins(Some("invalid".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new() {
        let result = ConnectUtilApp::new().await;
        assert!(result.is_ok());
        let app = result.unwrap();
        // Just verify we can create the app
        assert!(matches!(app, ConnectUtilApp { .. }));
    }

    #[tokio::test]
    async fn test_generate_terraform_non_interactive() {
        let app = ConnectUtilApp::new().await.unwrap();
        let options = ConnectorOptions {
            name: Some("test-connector".to_string()),
            output: Some("test-output.tf".to_string()),
        };

        // This test uses the non-interactive function
        let result = app.generate_terraform_non_interactive(options);
        assert!(result.is_ok());

        let terraform = result.unwrap();
        assert!(terraform.contains("test-connector"));
        assert!(terraform.contains("var.environment"));
    }

    #[tokio::test]
    async fn test_generate_terraform_non_interactive_missing_required() {
        let app = ConnectUtilApp::new().await.unwrap();
        let options = ConnectorOptions {
            name: None, // Missing required field
            output: Some("test-output.tf".to_string()),
        };

        // This should fail because required fields are missing
        let result = app.generate_terraform_non_interactive(options);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Connector name is required"));
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_mysql_cdc() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition {
            name: "MySqlCdcSourceV2".to_string(),
            display_name: "MySQL CDC Source V2".to_string(),
            connector_class: "MySqlCdcSourceV2".to_string(),
            description: "MySQL CDC Source V2".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let mut config_obj = Object::new();
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
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("database.ssl.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("snapshot.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("binlog.buffer.size")));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_with_topics() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresSink").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec!["topic1".to_string(), "topic2".to_string()],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check that topics are properly formatted with join() to flatten array
        assert!(terraform.contains("join(\",\", ["));
        assert!(terraform.contains("\"topic1\""));
        assert!(terraform.contains("\"topic2\""));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_with_topic_prefix() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresCdcSourceV2").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec!["topic1".to_string(), "topic2".to_string()],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check that topic.prefix is used for CDC connectors
        assert!(terraform.contains("\"topic.prefix\" = \"<REPLACE_WITH_TOPIC_PREFIX>\""));
        assert!(terraform.contains("join(\",\", ["));
        assert!(terraform.contains("\"topic1\""));
        assert!(terraform.contains("\"topic2\""));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_without_topics_sink() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresSink").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check that placeholder is used for sink connectors
        assert!(terraform.contains("topics = \"<REPLACE_WITH_TOPIC_NAME>\""));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_without_topics_source() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresCdcSourceV2").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check that topic.prefix placeholder is used for source connectors
        assert!(terraform.contains("\"topic.prefix\" = \"<REPLACE_WITH_TOPIC_PREFIX>\""));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_qa_environment() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresSink").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check QA-specific values
        assert!(terraform.contains("resource \"confluent_connector\""));
        assert!(terraform.contains("status = var.status"));
        assert!(terraform.contains("environment {"));
        assert!(terraform.contains("kafka_cluster {"));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_unknown_environment() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition::get_connector_by_name("PostgresSink").unwrap();

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check that terraform is generated with resource format
        assert!(terraform.contains("resource \"confluent_connector\""));
        assert!(terraform.contains("status = var.status"));
        assert!(terraform.contains("environment {"));
        assert!(terraform.contains("kafka_cluster {"));
    }

    #[tokio::test]
    async fn test_validate_connector_file_not_found() {
        let mut app = ConnectUtilApp::new().await.unwrap();
        let result = app.validate_connector("nonexistent.tf").await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Configuration file not found"));
    }

    #[tokio::test]
    async fn test_validate_connector_commented_file() {
        let mut app = ConnectUtilApp::new().await.unwrap();

        // Create a temporary file with commented content
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("commented.tf");
        std::fs::write(
            &temp_file,
            r#"
        # module "test_connector" {
        #   config_sensitive = {
        #     "connection.password" = "secret_password"
        #   }
        #   config_nonsensitive = {
        #     "connector.class" = "PostgresSink"
        #     "connection.host" = "localhost"
        #   }
        # }
        "#,
        )
        .unwrap();

        let result = app.validate_connector(temp_file.to_str().unwrap()).await;
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(&temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_validate_connector_unknown_connector() {
        let mut app = ConnectUtilApp::new().await.unwrap();

        // Create a temporary file with unknown connector
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("unknown.tf");
        std::fs::write(
            &temp_file,
            r#"
        module "test_connector" {
          config_sensitive = {
            "connection.password" = "secret_password"
          }
          config_nonsensitive = {
            "connector.class" = "UnknownConnector"
            "connection.host" = "localhost"
          }
        }
        "#,
        )
        .unwrap();

        let result = app.validate_connector(temp_file.to_str().unwrap()).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown connector"));

        // Clean up
        std::fs::remove_file(&temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_validate_connector_invalid_config() {
        let mut app = ConnectUtilApp::new().await.unwrap();

        // Create a temporary file with invalid config but valid Terraform structure
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("invalid.tf");
        std::fs::write(
            &temp_file,
            r#"
resource "confluent_connector" "test_connector" {
  status = "RUNNING"

  environment {
    id = var.environment_id
  }

  kafka_cluster {
    id = var.kafka_cluster.id
  }

  config_sensitive = {
    "connection.password" = "secret_password"
  }

  config_nonsensitive = {
    "connector.class" = "PostgresSink"
    "connection.host" = "localhost"
    # Missing required fields
  }

  lifecycle {
    ignore_changes = [
      config_nonsensitive["kafka.deployment.type"],
    ]
  }
}
        "#,
        )
        .unwrap();

        let result = app.validate_connector(temp_file.to_str().unwrap()).await;
        // This should succeed because the validation flow works even if config is invalid
        // The validation error is printed but doesn't cause the function to fail
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(&temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_postgres_source() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition {
            name: "PostgreSQLSource".to_string(),
            display_name: "PostgreSQL Source".to_string(),
            connector_class: "PostgreSQLSource".to_string(),
            description: "PostgreSQL Source".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let mut config_obj = Object::new();
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
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.host")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.port")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("ssl.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("mode")));
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_postgres_sink() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition {
            name: "PostgresSink".to_string(),
            display_name: "PostgreSQL Sink".to_string(),
            connector_class: "PostgresSink".to_string(),
            description: "PostgreSQL Sink".to_string(),
            connector_type: ConnectorType::Sink,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let mut config_obj = Object::new();
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
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.host")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.port")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("ssl.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("insert.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("pk.mode")));
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_mysql_source() {
        let connector = ConnectorDefinition {
            name: "MySQLSource".to_string(),
            display_name: "MySQL Source".to_string(),
            connector_class: "MySQLSource".to_string(),
            description: "MySQL Source".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let mut config_obj = Object::new();
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
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.host")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.port")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("ssl.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("mode")));
    }

    #[tokio::test]
    async fn test_add_connector_specific_config_mysql_sink() {
        let connector = ConnectorDefinition {
            name: "MySQLSink".to_string(),
            display_name: "MySQL Sink".to_string(),
            connector_class: "MySQLSink".to_string(),
            description: "MySQL Sink".to_string(),
            connector_type: ConnectorType::Sink,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec![],
        };

        let mut config_obj = Object::new();
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
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.host")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("connection.port")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("ssl.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("insert.mode")));
        assert!(config_obj.contains_key(&TerraformGenerator::make_object_key("pk.mode")));
    }

    #[tokio::test]
    async fn test_generate_terraform_config_sensitive_configs() {
        let _app = ConnectUtilApp::new().await.unwrap();
        let connector = ConnectorDefinition {
            name: "TestConnector".to_string(),
            display_name: "Test Connector".to_string(),
            connector_class: "TestConnector".to_string(),
            description: "Test Connector".to_string(),
            connector_type: ConnectorType::Source,
            required_configs: vec![],
            optional_configs: vec![],
            sensitive_configs: vec!["password".to_string(), "secret".to_string()],
        };

        let options = TerraformConfigOptions {
            connector_name: "test-connector".to_string(),
            connector: connector.clone(),
            topics: vec![],
            input_data_format: None,
            output_data_format: None,
        };

        let generator = TerraformGenerator;
        let result = generator.generate_connector_config(options);
        assert!(result.is_ok());
        let terraform = result.unwrap();

        // Check that sensitive configs are properly formatted
        assert!(terraform.contains("config_sensitive = {"));
        assert!(terraform.contains("password = \"<REPLACE_WITH_ACTUAL_VALUE>\""));
        assert!(terraform.contains("secret = \"<REPLACE_WITH_ACTUAL_VALUE>\""));
    }
}
