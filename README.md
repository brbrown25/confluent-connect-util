# Confluent Connect Utility

[![CI](https://github.com/brbrown25/confluent-connect-util/actions/workflows/ci.yml/badge.svg)](https://github.com/brbrown25/confluent-connect-util/actions/workflows/ci.yml)
[![Coverage](https://codecov.io/gh/brbrown25/confluent-connect-util/branch/main/graph/badge.svg)](https://codecov.io/gh/brbrown25/confluent-connect-util)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/brbrown25/confluent-connect-util?include_prereleases)](https://github.com/brbrown25/confluent-connect-util/releases)

A powerful CLI tool for validating and generating Confluent Kafka Connect connector configurations via Terraform. This utility simplifies the process of creating, validating, and managing Kafka Connect connectors in Confluent Cloud environments.

## Features

- 🎯 **Interactive Connector Generation**: Guided prompts for creating Terraform configurations for Kafka Connect connectors
- ✅ **Configuration Validation**: Validate existing connector configurations against connector definitions
- 🔍 **Plugin Discovery**: List and explore available connector plugins from Confluent Cloud
- 📦 **62+ Connector Types**: Support for all major Confluent Cloud connectors (source and sink)
- 🎨 **Fuzzy Search**: Interactive topic selection with fuzzy search capabilities
- 🔒 **Security**: Proper handling of sensitive configuration fields
- 📝 **Terraform Generation**: Generates Terraform configurations using HCL library for robust parsing and generation
- 🛠️ **Makefile Support**: Common development tasks via Makefile commands

## Installation

### From Releases

Download the appropriate binary for your platform from the [Releases](https://github.com/yourusername/confluent-connect-util/releases) page:

#### Linux
```bash
# x86_64
wget https://github.com/yourusername/confluent-connect-util/releases/download/v0.1.0/connect-util-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
tar -xzf connect-util-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
sudo mv connect-util /usr/local/bin/

# ARM64
wget https://github.com/yourusername/confluent-connect-util/releases/download/v0.1.0/connect-util-v0.1.0-aarch64-unknown-linux-gnu.tar.gz
tar -xzf connect-util-v0.1.0-aarch64-unknown-linux-gnu.tar.gz
sudo mv connect-util /usr/local/bin/
```

#### macOS
```bash
# Intel
curl -L https://github.com/yourusername/confluent-connect-util/releases/download/v0.1.0/connect-util-v0.1.0-x86_64-apple-darwin.tar.gz | tar -xz
sudo mv connect-util /usr/local/bin/

# Apple Silicon
curl -L https://github.com/yourusername/confluent-connect-util/releases/download/v0.1.0/connect-util-v0.1.0-aarch64-apple-darwin.tar.gz | tar -xz
sudo mv connect-util /usr/local/bin/
```

#### Windows
```powershell
# Download and extract the zip file
# Add connect-util.exe to your PATH
```

### From Source

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- Cargo (comes with Rust)

#### Build

```bash
# Clone the repository
git clone https://github.com/yourusername/confluent-connect-util.git
cd confluent-connect-util

# Build the release binary
cargo build --release

# The binary will be at target/release/connect-util (or connect-util.exe on Windows)
```

#### Install Locally

```bash
cargo install --path .
```

## Usage

### Basic Commands

#### List Available Plugins

List all available connector plugins:

```bash
connect-util list-plugins
```

Filter by connector type:

```bash
# List only source connectors
connect-util list-plugins --type source

# List only sink connectors
connect-util list-plugins --type sink
```

#### Generate Terraform Configuration

Generate a Terraform configuration interactively:

```bash
connect-util generate
```

The tool will guide you through:
1. Selecting the connector type (source or sink)
2. Choosing the specific connector plugin
3. Configuring connector-specific settings
4. Selecting topics (with fuzzy search)
5. Choosing input/output data format (AVRO, JSON_SR, PROTOBUF, JSON, PARQUET)
6. Specifying output file location

#### Generate with Command-Line Arguments

You can also provide arguments directly to skip some prompts:

```bash
connect-util generate \
  --name my-postgres-source \
  --output my-connector.tf
```

#### Validate Connector Configuration

Validate an existing Terraform connector configuration:

```bash
connect-util validate --config-file connector.tf
```

The validation checks:
- Connector configuration structure (required fields, sensitive fields)
- Terraform resource structure (status, environment, kafka_cluster blocks)
- Connector-specific configuration requirements

### Advanced Usage Examples

#### PostgreSQL CDC Source Connector

```bash
connect-util generate \
  --name postgres-cdc-source \
  --output postgres-cdc.tf
```

#### S3 Sink Connector

```bash
connect-util generate \
  --name s3-sink-connector \
  --output s3-sink.tf
```

#### MySQL Source Connector

```bash
connect-util generate \
  --name mysql-source \
  --output mysql-source.tf
```

## Command Reference

### `generate`

Generate Terraform configuration for a connector.

**Options:**
- `-n, --name <NAME>`: Connector name (optional - will prompt if not provided)
- `-o, --output <OUTPUT>`: Output file path (optional - will prompt if not provided)

**Example:**
```bash
connect-util generate --name my-connector --output my-connector.tf
```

### `validate`

Validate a connector configuration file.

**Options:**
- `-c, --config-file <CONFIG_FILE>`: Connector configuration file (required)

**Example:**
```bash
connect-util validate --config-file connector.tf
```

### `list-plugins`

List available connector plugins.

**Options:**
- `-t, --type <TYPE>`: Filter by connector type (source, sink) (optional)

**Example:**
```bash
connect-util list-plugins --type source
```

## Generated Terraform Structure

The tool generates Terraform configurations using direct `confluent_connector` resources:

```terraform
resource "confluent_connector" "my_connector" {
  status = var.status

  environment {
    id = var.environment_id
  }

  kafka_cluster {
    id = var.kafka_cluster.id
  }

  config_sensitive = {
    password = "<REPLACE_WITH_ACTUAL_VALUE>"
  }

  config_nonsensitive = {
    "connector.class" = "io.debezium.connector.postgresql.PostgresConnector"
    name              = "my-connector"
    topics            = join(",", ["topic1", "topic2"])
    "database.hostname" = "<REPLACE_WITH_DATABASE_HOST>"
    "database.port"     = "5432"
    "output.data.format" = local.schema_formats.avro
    # ... other connector-specific configuration
  }

  lifecycle {
    ignore_changes = [
      config_nonsensitive["kafka.deployment.type"],
      config_nonsensitive["kafka.max.partition.validation.disable"],
      config_nonsensitive["kafka.max.partition.validation.enable"],
      config_nonsensitive["kafka.max.partition.validation"],
    ]
  }
}
```

## Supported Connectors

The tool supports 62+ connector types including:

### Source Connectors
- **PostgresCdcSourceV2**: PostgreSQL Change Data Capture
- **MySqlCdcSourceV2**: MySQL Change Data Capture
- **JdbcSourceConnector**: Generic JDBC source
- **HttpSourceConnector**: HTTP API polling
- **AmazonS3Source**: Amazon S3 source
- **GoogleCloudPubSubSource**: Google Cloud Pub/Sub
- **AzureEventHubsSource**: Azure Event Hubs
- **GitHubSource**: GitHub integration
- **DatagenSource**: Data generation for testing
- And many more...

### Sink Connectors
- **S3_SINK**: Amazon S3 storage
- **PostgresSink**: PostgreSQL sink
- **MySqlSink**: MySQL sink
- **ElasticsearchSink**: Elasticsearch indexing
- **MongoDBSink**: MongoDB storage
- **BigQuerySink**: Google BigQuery
- **SnowflakeSink**: Snowflake data warehouse
- **ClickHouseSink**: ClickHouse analytics database
- **SplunkSink**: Splunk logging platform
- And many more...

Run `connect-util list-plugins` to see the complete list.

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo
- Make (optional, for using Makefile commands)

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/confluent-connect-util.git
cd confluent-connect-util

# Install dependencies
cargo build
# or use Makefile
make build
```

### Using the Makefile

The project includes a Makefile with common development commands:

```bash
# Show all available commands
make help

# Build the project (debug)
make build
# or
make debug

# Build release
make release

# Run all tests
make test

# Run tests with verbose output
make test-verbose

# Run tests in watch mode (requires cargo-watch)
make test-watch

# Format code
make fmt

# Check formatting without changes
make fmt-check

# Run linter
make lint

# Auto-fix linting issues
make lint-fix

# Generate coverage report (requires cargo-tarpaulin)
make coverage

# Generate HTML coverage report
make coverage-html

# Check formatting and linting
make check

# Run all checks (format, lint, test)
make all

# Run CI checks
make ci

# Clean build artifacts
make clean

# Install development tools
make install-tools
```

### Running Tests

```bash
# Using Makefile (recommended)
make test

# Or using cargo directly
cargo test

# Run tests with output
cargo test -- --nocapture
# or
make test-verbose

# Run tests for a specific module
cargo test --lib
```

### Code Coverage

```bash
# Using Makefile
make coverage        # XML report
make coverage-html    # HTML report

# Or using cargo-tarpaulin directly
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Linting

```bash
# Using Makefile (recommended)
make lint        # Check for issues
make lint-fix    # Auto-fix issues

# Or using cargo directly
cargo fmt --check
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

### Building

```bash
# Using Makefile
make build       # Debug build
make release     # Release build

# Or using cargo directly
cargo build
cargo build --release
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

- **CI Workflow**: Runs on every push and pull request
  - Linting (rustfmt, clippy)
  - Testing across multiple platforms
  - Code coverage reporting

- **Release Workflow**: Triggers on version tags
  - Cross-platform builds (Linux, macOS, Windows)
  - Automatic release creation
  - Support for alpha and official releases

### Release Process

#### Alpha Releases

Tag with `-alpha` suffix:

```bash
git tag v0.1.0-alpha.1
git push origin v0.1.0-alpha.1
```

#### Official Releases

Tag with version number:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow will automatically:
1. Build binaries for all supported platforms
2. Create a GitHub release
3. Upload all binaries and checksums
4. Mark alpha releases as pre-releases

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting (`cargo test && cargo clippy`)
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Follow Rust conventions and best practices
- Write tests for new features
- Update documentation as needed
- Ensure all tests pass and linting checks succeed
- Keep commits focused and well-described

## Terraform Resource Structure

The generated Terraform uses direct `confluent_connector` resources with:

- **Status**: Variable reference (`var.status`)
- **Environment**: Block with `id` attribute (`var.environment_id`)
- **Kafka Cluster**: Block with `id` attribute (`var.kafka_cluster.id`)
- **Config Maps**: Separate `config_sensitive` and `config_nonsensitive` maps
- **Lifecycle**: Automatic ignore_changes for auto-managed Confluent Cloud attributes
- **Topics**: Uses `join()` function to flatten arrays into comma-separated strings
- **Data Formats**: References to `local.schema_formats.*` for schema registry formats

## Security Considerations

- No API keys or credentials are stored by the tool
- All sensitive values are marked as placeholders in generated Terraform
- Proper error handling for validation failures
- Support for service account-based authentication patterns

## Troubleshooting

### Common Issues

**Issue**: "Command not found: connect-util"
- **Solution**: Ensure the binary is in your PATH or use the full path to the binary

**Issue**: "Failed to parse Terraform file"
- **Solution**: Ensure the Terraform file is valid and follows the expected structure

**Issue**: "Connector plugin not found"
- **Solution**: Verify the connector name is correct using `connect-util list-plugins`

**Issue**: "Failed to validate Terraform structure"
- **Solution**: Ensure the Terraform file uses `resource "confluent_connector"` with proper `environment` and `kafka_cluster` blocks

## License

This project is licensed under the terms specified in the [LICENSE](LICENSE) file.

## Support

For issues, questions, or contributions, please open an issue on the [GitHub repository](https://github.com/yourusername/confluent-connect-util/issues).

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [dialoguer](https://github.com/console-rs/dialoguer) for interactive prompts
- Uses [hcl-rs](https://github.com/martinohmann/hcl-rs) for HCL parsing and generation
- Integrates with Confluent Cloud APIs
