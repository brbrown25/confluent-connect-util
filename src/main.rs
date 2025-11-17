use clap::{Parser, Subcommand};
use connect_util::{app::ConnectUtilApp, error::ConnectUtilError, types::ConnectorOptions};
use tracing::info;

#[derive(Parser)]
#[command(name = "connect-util")]
#[command(about = "Interactive Kafka Connect Connector Terraform Generator")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Terraform configuration interactively
    Generate {
        /// Connector name (optional - will prompt if not provided)
        #[arg(short, long)]
        name: Option<String>,

        /// Output file path (optional - will prompt if not provided)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Validate a connector configuration
    Validate {
        /// Connector configuration file
        #[arg(short, long)]
        config_file: String,
    },

    /// List available connector plugins
    ListPlugins {
        /// Filter by connector type (source, sink)
        #[arg(short, long)]
        r#type: Option<String>,
    },
}

#[cfg(not(tarpaulin_include))]
#[tokio::main]
async fn main() -> Result<(), ConnectUtilError> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let mut app = ConnectUtilApp::new().await?;

    match cli.command {
        Commands::Generate { name, output } => {
            info!("Starting interactive Terraform generation");
            let options = ConnectorOptions { name, output };
            app.generate_terraform_interactive(options).await?;
        }

        Commands::Validate { config_file } => {
            info!("Validating connector configuration");
            app.validate_connector(&config_file).await?;
        }

        Commands::ListPlugins { r#type } => {
            info!("Listing available connector plugins");
            app.list_plugins(r#type).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parsing_generate_command() {
        let cli = Cli::try_parse_from([
            "connect-util",
            "generate",
            "--name",
            "test-connector",
            "--output",
            "test-output.tf",
        ])
        .unwrap();

        match cli.command {
            Commands::Generate { name, output } => {
                assert_eq!(name, Some("test-connector".to_string()));
                assert_eq!(output, Some("test-output.tf".to_string()));
            }
            _ => panic!("Expected Generate command"),
        }
    }

    #[test]
    fn test_cli_parsing_generate_command_minimal() {
        let cli = Cli::try_parse_from(["connect-util", "generate"]).unwrap();

        match cli.command {
            Commands::Generate { name, output } => {
                assert_eq!(name, None);
                assert_eq!(output, None);
            }
            _ => panic!("Expected Generate command"),
        }
    }

    #[test]
    fn test_cli_parsing_validate_command() {
        let cli = Cli::try_parse_from([
            "connect-util",
            "validate",
            "--config-file",
            "test-config.tf",
        ])
        .unwrap();

        match cli.command {
            Commands::Validate { config_file } => {
                assert_eq!(config_file, "test-config.tf");
            }
            _ => panic!("Expected Validate command"),
        }
    }

    #[test]
    fn test_cli_parsing_list_plugins_command() {
        let cli =
            Cli::try_parse_from(["connect-util", "list-plugins", "--type", "source"]).unwrap();

        match cli.command {
            Commands::ListPlugins { r#type } => {
                assert_eq!(r#type, Some("source".to_string()));
            }
            _ => panic!("Expected ListPlugins command"),
        }
    }

    #[test]
    fn test_cli_parsing_list_plugins_command_minimal() {
        let cli = Cli::try_parse_from(["connect-util", "list-plugins"]).unwrap();

        match cli.command {
            Commands::ListPlugins { r#type } => {
                assert_eq!(r#type, None);
            }
            _ => panic!("Expected ListPlugins command"),
        }
    }

    #[test]
    fn test_cli_help_generation() {
        let mut cli = Cli::command();
        let help = cli.render_help().to_string();

        assert!(help.contains("Interactive Kafka Connect Connector Terraform Generator"));
        assert!(help.contains("generate"));
        assert!(help.contains("validate"));
        assert!(help.contains("list-plugins"));
        assert!(help.contains("Usage: connect-util"));
    }

    #[test]
    fn test_cli_generate_help() {
        let mut cli = Cli::command();
        let help = cli.render_help().to_string();

        assert!(help.contains("Generate Terraform configuration interactively"));
        assert!(help.contains("generate"));
    }

    #[test]
    fn test_cli_validate_help() {
        let mut cli = Cli::command();
        let help = cli.render_help().to_string();

        assert!(help.contains("Validate a connector configuration"));
        assert!(help.contains("validate"));
    }

    #[test]
    fn test_cli_list_plugins_help() {
        let mut cli = Cli::command();
        let help = cli.render_help().to_string();

        assert!(help.contains("List available connector plugins"));
        assert!(help.contains("list-plugins"));
    }

    #[test]
    fn test_cli_invalid_command() {
        let result = Cli::try_parse_from(["connect-util", "invalid-command"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_cli_missing_required_arg() {
        let result = Cli::try_parse_from(["connect-util", "validate"]);

        assert!(result.is_err());
    }

    #[test]
    fn test_cli_short_args() {
        let cli = Cli::try_parse_from([
            "connect-util",
            "generate",
            "-n",
            "test-connector",
            "-o",
            "test.tf",
        ])
        .unwrap();

        match cli.command {
            Commands::Generate { name, output } => {
                assert_eq!(name, Some("test-connector".to_string()));
                assert_eq!(output, Some("test.tf".to_string()));
            }
            _ => panic!("Expected Generate command"),
        }
    }

    #[test]
    fn test_cli_validate_short_args() {
        let cli =
            Cli::try_parse_from(["connect-util", "validate", "-c", "test-config.tf"]).unwrap();

        match cli.command {
            Commands::Validate { config_file } => {
                assert_eq!(config_file, "test-config.tf");
            }
            _ => panic!("Expected Validate command"),
        }
    }

    #[test]
    fn test_cli_list_plugins_short_args() {
        let cli = Cli::try_parse_from(["connect-util", "list-plugins", "-t", "sink"]).unwrap();

        match cli.command {
            Commands::ListPlugins { r#type } => {
                assert_eq!(r#type, Some("sink".to_string()));
            }
            _ => panic!("Expected ListPlugins command"),
        }
    }

    #[test]
    fn test_cli_version() {
        let cli = Cli::command();
        let version = cli.render_version().to_string();

        assert!(version.contains("0.1.0"));
    }
}
