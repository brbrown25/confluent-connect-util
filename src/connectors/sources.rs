use super::config_field;
use crate::types::{ConnectorDefinition, ConnectorType};

// Source Connectors
pub(crate) fn activemq_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "ActiveMQSource".to_string(),
        display_name: "ActiveMQ Source".to_string(),
        connector_class: "ActiveMQSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from ActiveMQ queues".to_string(),
        required_configs: vec![
            config_field(
                "activemq.broker.url",
                "ActiveMQ broker URL",
                "string",
                true,
                None,
            ),
            config_field(
                "activemq.queue.name",
                "ActiveMQ queue name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "activemq.username",
                "ActiveMQ username",
                "string",
                false,
                None,
            ),
            config_field(
                "activemq.password",
                "ActiveMQ password",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "activemq.client.id",
                "ActiveMQ client ID",
                "string",
                false,
                None,
            ),
            config_field(
                "activemq.session.acknowledge.mode",
                "ActiveMQ session acknowledge mode",
                "string",
                false,
                Some(vec![
                    "AUTO_ACKNOWLEDGE".to_string(),
                    "CLIENT_ACKNOWLEDGE".to_string(),
                    "DUPS_OK_ACKNOWLEDGE".to_string(),
                ]),
            ),
            config_field(
                "activemq.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["activemq.password".to_string()],
    }
}

pub(crate) fn amazon_cloudwatch_logs_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AmazonCloudWatchLogsSource".to_string(),
        display_name: "Amazon CloudWatch Logs Source".to_string(),
        connector_class: "AmazonCloudWatchLogsSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read log data from Amazon CloudWatch Logs".to_string(),
        required_configs: vec![
            config_field("aws.region", "AWS region", "string", true, None),
            config_field(
                "log.group.name",
                "CloudWatch log group name",
                "string",
                true,
                None,
            ),
            config_field(
                "log.stream.name",
                "CloudWatch log stream name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "aws.access.key.id",
                "AWS access key ID",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.secret.access.key",
                "AWS secret access key",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.session.token",
                "AWS session token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "start.time",
                "Start time for log reading",
                "string",
                false,
                None,
            ),
            config_field(
                "end.time",
                "End time for log reading",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "aws.access.key.id".to_string(),
            "aws.secret.access.key".to_string(),
            "aws.session.token".to_string(),
        ],
    }
}

pub(crate) fn amazon_dynamodb_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AmazonDynamoDBCdcSource".to_string(),
        display_name: "Amazon DynamoDB CDC Source".to_string(),
        connector_class: "AmazonDynamoDBCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from Amazon DynamoDB tables".to_string(),
        required_configs: vec![
            config_field("aws.region", "AWS region", "string", true, None),
            config_field("table.name", "DynamoDB table name", "string", true, None),
        ],
        optional_configs: vec![
            config_field(
                "aws.access.key.id",
                "AWS access key ID",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.secret.access.key",
                "AWS secret access key",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.session.token",
                "AWS session token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "stream.view.type",
                "DynamoDB stream view type",
                "string",
                false,
                Some(vec![
                    "NEW_AND_OLD_IMAGES".to_string(),
                    "NEW_IMAGES".to_string(),
                    "OLD_IMAGES".to_string(),
                    "KEYS_ONLY".to_string(),
                ]),
            ),
        ],
        sensitive_configs: vec![
            "aws.access.key.id".to_string(),
            "aws.secret.access.key".to_string(),
            "aws.session.token".to_string(),
        ],
    }
}

pub(crate) fn amazon_kinesis_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AmazonKinesisSource".to_string(),
        display_name: "Amazon Kinesis Source".to_string(),
        connector_class: "AmazonKinesisSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Amazon Kinesis streams".to_string(),
        required_configs: vec![
            config_field("aws.region", "AWS region", "string", true, None),
            config_field(
                "kinesis.stream.name",
                "Kinesis stream name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "aws.access.key.id",
                "AWS access key ID",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.secret.access.key",
                "AWS secret access key",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.session.token",
                "AWS session token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "shard.iterator.type",
                "Shard iterator type",
                "string",
                false,
                Some(vec![
                    "TRIM_HORIZON".to_string(),
                    "LATEST".to_string(),
                    "AT_TIMESTAMP".to_string(),
                ]),
            ),
            config_field(
                "timestamp",
                "Timestamp for AT_TIMESTAMP iterator",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "aws.access.key.id".to_string(),
            "aws.secret.access.key".to_string(),
            "aws.session.token".to_string(),
        ],
    }
}

pub(crate) fn amazon_s3_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AmazonS3Source".to_string(),
        display_name: "Amazon S3 Source".to_string(),
        connector_class: "AmazonS3Source".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Amazon S3 buckets".to_string(),
        required_configs: vec![
            config_field("aws.region", "AWS region", "string", true, None),
            config_field("s3.bucket.name", "S3 bucket name", "string", true, None),
            config_field(
                "s3.object.key",
                "S3 object key pattern",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "aws.access.key.id",
                "AWS access key ID",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.secret.access.key",
                "AWS secret access key",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.session.token",
                "AWS session token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("s3.prefix", "S3 object prefix", "string", false, None),
            config_field("s3.delimiter", "S3 object delimiter", "string", false, None),
        ],
        sensitive_configs: vec![
            "aws.access.key.id".to_string(),
            "aws.secret.access.key".to_string(),
            "aws.session.token".to_string(),
        ],
    }
}

pub(crate) fn amazon_sqs_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AmazonSQSSource".to_string(),
        display_name: "Amazon SQS Source".to_string(),
        connector_class: "AmazonSQSSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from Amazon SQS queues".to_string(),
        required_configs: vec![
            config_field("aws.region", "AWS region", "string", true, None),
            config_field("sqs.queue.url", "SQS queue URL", "string", true, None),
        ],
        optional_configs: vec![
            config_field(
                "aws.access.key.id",
                "AWS access key ID",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.secret.access.key",
                "AWS secret access key",
                "string",
                false,
                None,
            ),
            config_field(
                "aws.session.token",
                "AWS session token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "sqs.visibility.timeout",
                "SQS visibility timeout",
                "int",
                false,
                None,
            ),
            config_field(
                "sqs.wait.time.seconds",
                "SQS wait time in seconds",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "aws.access.key.id".to_string(),
            "aws.secret.access.key".to_string(),
            "aws.session.token".to_string(),
        ],
    }
}

pub(crate) fn azure_blob_storage_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AzureBlobStorageSource".to_string(),
        display_name: "Azure Blob Storage Source".to_string(),
        connector_class: "AzureBlobStorageSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Azure Blob Storage".to_string(),
        required_configs: vec![
            config_field(
                "azure.storage.account.name",
                "Azure storage account name",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.storage.container.name",
                "Azure storage container name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "azure.storage.account.key",
                "Azure storage account key",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.storage.connection.string",
                "Azure storage connection string",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "azure.blob.prefix",
                "Azure blob prefix",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.blob.delimiter",
                "Azure blob delimiter",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "azure.storage.account.key".to_string(),
            "azure.storage.connection.string".to_string(),
        ],
    }
}

pub(crate) fn azure_cosmos_db_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AzureCosmosDBSource".to_string(),
        display_name: "Azure Cosmos DB Source".to_string(),
        connector_class: "AzureCosmosDBSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Azure Cosmos DB".to_string(),
        required_configs: vec![
            config_field(
                "azure.cosmosdb.endpoint",
                "Azure Cosmos DB endpoint",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.cosmosdb.database.name",
                "Azure Cosmos DB database name",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.cosmosdb.container.name",
                "Azure Cosmos DB container name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "azure.cosmosdb.key",
                "Azure Cosmos DB key",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.connection.string",
                "Azure Cosmos DB connection string",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.query",
                "Azure Cosmos DB query",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.partition.key",
                "Azure Cosmos DB partition key",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "azure.cosmosdb.key".to_string(),
            "azure.cosmosdb.connection.string".to_string(),
        ],
    }
}

pub(crate) fn azure_cosmos_db_source_v2() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AzureCosmosDBSourceV2".to_string(),
        display_name: "Azure Cosmos DB Source V2".to_string(),
        connector_class: "AzureCosmosDBSourceV2".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Azure Cosmos DB (V2)".to_string(),
        required_configs: vec![
            config_field(
                "azure.cosmosdb.endpoint",
                "Azure Cosmos DB endpoint",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.cosmosdb.database.name",
                "Azure Cosmos DB database name",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.cosmosdb.container.name",
                "Azure Cosmos DB container name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "azure.cosmosdb.key",
                "Azure Cosmos DB key",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.connection.string",
                "Azure Cosmos DB connection string",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.query",
                "Azure Cosmos DB query",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.partition.key",
                "Azure Cosmos DB partition key",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.cosmosdb.change.feed",
                "Enable change feed",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec![
            "azure.cosmosdb.key".to_string(),
            "azure.cosmosdb.connection.string".to_string(),
        ],
    }
}

pub(crate) fn azure_event_hubs_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AzureEventHubsSource".to_string(),
        display_name: "Azure Event Hubs Source".to_string(),
        connector_class: "AzureEventHubsSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Azure Event Hubs".to_string(),
        required_configs: vec![
            config_field(
                "azure.eventhubs.namespace",
                "Azure Event Hubs namespace",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.eventhubs.hub.name",
                "Azure Event Hubs hub name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "azure.eventhubs.connection.string",
                "Azure Event Hubs connection string",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.eventhubs.sas.key.name",
                "Azure Event Hubs SAS key name",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.eventhubs.sas.key",
                "Azure Event Hubs SAS key",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "azure.eventhubs.consumer.group",
                "Azure Event Hubs consumer group",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.eventhubs.partition.count",
                "Azure Event Hubs partition count",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "azure.eventhubs.connection.string".to_string(),
            "azure.eventhubs.sas.key".to_string(),
        ],
    }
}

pub(crate) fn azure_service_bus_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "AzureServiceBusSource".to_string(),
        display_name: "Azure Service Bus Source".to_string(),
        connector_class: "AzureServiceBusSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from Azure Service Bus".to_string(),
        required_configs: vec![
            config_field(
                "azure.servicebus.namespace",
                "Azure Service Bus namespace",
                "string",
                true,
                None,
            ),
            config_field(
                "azure.servicebus.queue.name",
                "Azure Service Bus queue name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "azure.servicebus.connection.string",
                "Azure Service Bus connection string",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.servicebus.sas.key.name",
                "Azure Service Bus SAS key name",
                "string",
                false,
                None,
            ),
            config_field(
                "azure.servicebus.sas.key",
                "Azure Service Bus SAS key",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "azure.servicebus.max.delivery.count",
                "Azure Service Bus max delivery count",
                "int",
                false,
                None,
            ),
            config_field(
                "azure.servicebus.lock.duration",
                "Azure Service Bus lock duration",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "azure.servicebus.connection.string".to_string(),
            "azure.servicebus.sas.key".to_string(),
        ],
    }
}

pub(crate) fn couchbase_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "CouchbaseSource".to_string(),
        display_name: "Couchbase Source".to_string(),
        connector_class: "CouchbaseSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Couchbase".to_string(),
        required_configs: vec![
            config_field(
                "couchbase.hostname",
                "Couchbase hostname",
                "string",
                true,
                None,
            ),
            config_field("couchbase.port", "Couchbase port", "int", true, None),
            config_field(
                "couchbase.username",
                "Couchbase username",
                "string",
                true,
                None,
            ),
            config_field(
                "couchbase.bucket.name",
                "Couchbase bucket name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "couchbase.password",
                "Couchbase password",
                "string",
                false,
                None,
            ),
            config_field(
                "couchbase.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("couchbase.query", "Couchbase query", "string", false, None),
            config_field(
                "couchbase.scan.consistency",
                "Couchbase scan consistency",
                "string",
                false,
                Some(vec![
                    "not_bounded".to_string(),
                    "request_plus".to_string(),
                    "statement_plus".to_string(),
                ]),
            ),
        ],
        sensitive_configs: vec!["couchbase.password".to_string()],
    }
}

pub(crate) fn datagen_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "DatagenSource".to_string(),
        display_name: "Datagen Source (development and testing)".to_string(),
        connector_class: "DatagenSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Generate test data for development and testing".to_string(),
        required_configs: vec![
            config_field(
                "kafka.topic",
                "Kafka topic to write to",
                "string",
                true,
                None,
            ),
            config_field(
                "quickstart",
                "Quickstart template to use",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "max.interval",
                "Maximum interval between messages (ms)",
                "int",
                false,
                None,
            ),
            config_field(
                "iterations",
                "Number of iterations to run",
                "int",
                false,
                None,
            ),
            config_field("tasks.max", "Maximum number of tasks", "int", false, None),
            config_field("schema.filename", "Schema filename", "string", false, None),
            config_field("schema.keyfield", "Schema key field", "string", false, None),
            config_field(
                "schema.stringfield",
                "Schema string field",
                "string",
                false,
                None,
            ),
            config_field(
                "schema.fieldname",
                "Schema field name",
                "string",
                false,
                None,
            ),
            config_field(
                "schema.fieldtype",
                "Schema field type",
                "string",
                false,
                None,
            ),
            config_field(
                "schema.fieldlength",
                "Schema field length",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![],
    }
}

pub(crate) fn github_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "GitHubSource".to_string(),
        display_name: "GitHub Source".to_string(),
        connector_class: "GitHubSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from GitHub repositories".to_string(),
        required_configs: vec![
            config_field(
                "github.owner",
                "GitHub repository owner",
                "string",
                true,
                None,
            ),
            config_field(
                "github.repo",
                "GitHub repository name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "github.token",
                "GitHub personal access token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("github.api.url", "GitHub API URL", "string", false, None),
            config_field(
                "github.events",
                "GitHub events to track",
                "string",
                false,
                None,
            ),
            config_field(
                "github.since",
                "GitHub events since timestamp",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["github.token".to_string()],
    }
}

pub(crate) fn google_cloud_pubsub_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "GoogleCloudPubSubSource".to_string(),
        display_name: "Google Cloud Pub/Sub Source".to_string(),
        connector_class: "GoogleCloudPubSubSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from Google Cloud Pub/Sub".to_string(),
        required_configs: vec![
            config_field(
                "gcp.project.id",
                "Google Cloud project ID",
                "string",
                true,
                None,
            ),
            config_field(
                "gcp.pubsub.subscription",
                "Google Cloud Pub/Sub subscription",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "gcp.credentials.json",
                "Google Cloud credentials JSON",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "gcp.pubsub.max.ack.deadline",
                "Google Cloud Pub/Sub max ack deadline",
                "int",
                false,
                None,
            ),
            config_field(
                "gcp.pubsub.parallel.pull",
                "Google Cloud Pub/Sub parallel pull",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["gcp.credentials.json".to_string()],
    }
}

pub(crate) fn http_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "HttpSource".to_string(),
        display_name: "HTTP Source".to_string(),
        connector_class: "HttpSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from HTTP endpoints".to_string(),
        required_configs: vec![config_field(
            "http.url",
            "HTTP endpoint URL",
            "string",
            true,
            None,
        )],
        optional_configs: vec![
            config_field(
                "http.auth.username",
                "HTTP authentication username",
                "string",
                false,
                None,
            ),
            config_field(
                "http.auth.password",
                "HTTP authentication password",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "http.method",
                "HTTP method",
                "string",
                false,
                Some(vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ]),
            ),
            config_field("http.headers", "HTTP headers", "string", false, None),
            config_field(
                "http.timeout.ms",
                "HTTP timeout in milliseconds",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["http.auth.password".to_string()],
    }
}

pub(crate) fn http_source_v2() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "HttpSourceV2".to_string(),
        display_name: "HTTP Source V2".to_string(),
        connector_class: "HttpSourceV2".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from HTTP endpoints (V2)".to_string(),
        required_configs: vec![config_field(
            "http.url",
            "HTTP endpoint URL",
            "string",
            true,
            None,
        )],
        optional_configs: vec![
            config_field(
                "http.auth.username",
                "HTTP authentication username",
                "string",
                false,
                None,
            ),
            config_field(
                "http.auth.password",
                "HTTP authentication password",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "http.method",
                "HTTP method",
                "string",
                false,
                Some(vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ]),
            ),
            config_field("http.headers", "HTTP headers", "string", false, None),
            config_field(
                "http.timeout.ms",
                "HTTP timeout in milliseconds",
                "int",
                false,
                None,
            ),
            config_field("http.retry.count", "HTTP retry count", "int", false, None),
        ],
        sensitive_configs: vec!["http.auth.password".to_string()],
    }
}

pub(crate) fn ibm_mq_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "IBMMQSource".to_string(),
        display_name: "IBM MQ Source".to_string(),
        connector_class: "IBMMQSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from IBM MQ".to_string(),
        required_configs: vec![
            config_field("ibm.mq.hostname", "IBM MQ hostname", "string", true, None),
            config_field("ibm.mq.port", "IBM MQ port", "int", true, None),
            config_field(
                "ibm.mq.queue.manager",
                "IBM MQ queue manager",
                "string",
                true,
                None,
            ),
            config_field(
                "ibm.mq.queue.name",
                "IBM MQ queue name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field("ibm.mq.username", "IBM MQ username", "string", false, None),
            config_field("ibm.mq.password", "IBM MQ password", "string", false, None),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("ibm.mq.channel", "IBM MQ channel", "string", false, None),
            config_field(
                "ibm.mq.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["ibm.mq.password".to_string()],
    }
}

pub(crate) fn influxdb_2_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "InfluxDB2Source".to_string(),
        display_name: "InfluxDB 2 Source".to_string(),
        connector_class: "InfluxDB2Source".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from InfluxDB 2".to_string(),
        required_configs: vec![
            config_field("influxdb.url", "InfluxDB URL", "string", true, None),
            config_field(
                "influxdb.org",
                "InfluxDB organization",
                "string",
                true,
                None,
            ),
            config_field("influxdb.bucket", "InfluxDB bucket", "string", true, None),
        ],
        optional_configs: vec![
            config_field("influxdb.token", "InfluxDB token", "string", false, None),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("influxdb.query", "InfluxDB query", "string", false, None),
            config_field(
                "influxdb.start",
                "InfluxDB start time",
                "string",
                false,
                None,
            ),
            config_field("influxdb.stop", "InfluxDB stop time", "string", false, None),
        ],
        sensitive_configs: vec!["influxdb.token".to_string()],
    }
}

pub(crate) fn jira_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "JiraSource".to_string(),
        display_name: "Jira Source".to_string(),
        connector_class: "JiraSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Jira".to_string(),
        required_configs: vec![
            config_field("jira.url", "Jira URL", "string", true, None),
            config_field("jira.username", "Jira username", "string", true, None),
        ],
        optional_configs: vec![
            config_field("jira.password", "Jira password", "string", false, None),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("jira.jql", "Jira JQL query", "string", false, None),
            config_field(
                "jira.fields",
                "Jira fields to retrieve",
                "string",
                false,
                None,
            ),
            config_field("jira.expand", "Jira expand options", "string", false, None),
        ],
        sensitive_configs: vec!["jira.password".to_string()],
    }
}

pub(crate) fn mariadb_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MariaDBCdcSource".to_string(),
        display_name: "MariaDB CDC Source".to_string(),
        connector_class: "MariaDBCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from MariaDB databases".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field("database.dbname", "Database name", "string", true, None),
            config_field(
                "topic.prefix",
                "Topic prefix for CDC events",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "database.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                ]),
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn microsoft_sql_server_cdc_source_v2() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MicrosoftSqlServerCdcSourceV2".to_string(),
        display_name: "Microsoft SQL Server CDC Source V2 (Debezium)".to_string(),
        connector_class: "MicrosoftSqlServerCdcSourceV2".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from Microsoft SQL Server databases (V2)".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field("database.dbname", "Database name", "string", true, None),
            config_field(
                "topic.prefix",
                "Topic prefix for CDC events",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "database.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                ]),
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
            config_field(
                "database.server.name",
                "Database server name",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn microsoft_sql_server_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MicrosoftSqlServerSource".to_string(),
        display_name: "Microsoft SQL Server Source (JDBC)".to_string(),
        connector_class: "MicrosoftSqlServerSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Microsoft SQL Server databases".to_string(),
        required_configs: vec![
            config_field("connection.host", "Database hostname", "string", true, None),
            config_field("connection.port", "Database port", "int", true, None),
            config_field("connection.user", "Database username", "string", true, None),
            config_field("db.name", "Database name", "string", true, None),
            config_field(
                "table.whitelist",
                "Comma-separated list of tables to include",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "connection.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "connection.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "table.blacklist",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field("query", "Custom SQL query", "string", false, None),
            config_field(
                "incrementing.column.name",
                "Incrementing column name",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["connection.password".to_string()],
    }
}

pub(crate) fn mongodb_atlas_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MongoDBAtlasSource".to_string(),
        display_name: "MongoDB Atlas Source".to_string(),
        connector_class: "MongoDBAtlasSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from MongoDB Atlas".to_string(),
        required_configs: vec![
            config_field("mongodb.host", "MongoDB hostname", "string", true, None),
            config_field("mongodb.port", "MongoDB port", "int", true, None),
            config_field("mongodb.username", "MongoDB username", "string", true, None),
            config_field(
                "mongodb.database",
                "MongoDB database name",
                "string",
                true,
                None,
            ),
            config_field(
                "mongodb.collection",
                "MongoDB collection name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "mongodb.password",
                "MongoDB password",
                "string",
                false,
                None,
            ),
            config_field(
                "mongodb.auth.source",
                "MongoDB authentication source",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("mongodb.query", "MongoDB query", "string", false, None),
            config_field(
                "mongodb.projection",
                "MongoDB projection",
                "string",
                false,
                None,
            ),
            config_field(
                "mongodb.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["mongodb.password".to_string()],
    }
}

pub(crate) fn mqtt_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MQTTSource".to_string(),
        display_name: "MQTT Source".to_string(),
        connector_class: "MQTTSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from MQTT brokers".to_string(),
        required_configs: vec![
            config_field("mqtt.broker.url", "MQTT broker URL", "string", true, None),
            config_field(
                "mqtt.topics",
                "MQTT topics to subscribe to",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field("mqtt.username", "MQTT username", "string", false, None),
            config_field("mqtt.password", "MQTT password", "string", false, None),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("mqtt.client.id", "MQTT client ID", "string", false, None),
            config_field(
                "mqtt.qos",
                "MQTT QoS level",
                "int",
                false,
                Some(vec!["0".to_string(), "1".to_string(), "2".to_string()]),
            ),
            config_field(
                "mqtt.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["mqtt.password".to_string()],
    }
}

pub(crate) fn mysql_cdc_source_v2() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MySqlCdcSourceV2".to_string(),
        display_name: "MySQL CDC Source V2 (Debezium)".to_string(),
        connector_class: "MySqlCdcSourceV2".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from MySQL databases (V2)".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field(
                "database.server.name",
                "Logical name for the MySQL server",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.ssl.mode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disabled".to_string(),
                    "preferred".to_string(),
                    "required".to_string(),
                    "verify-ca".to_string(),
                    "verify-identity".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                    "schema_only".to_string(),
                ]),
            ),
            config_field(
                "binlog.buffer.size",
                "Size of the buffer used for binlog events",
                "int",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
            config_field(
                "max.queue.size",
                "Maximum number of records to queue",
                "int",
                false,
                None,
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "connect.timeout.ms",
                "Connection timeout in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "socket.timeout.ms",
                "Socket timeout in milliseconds",
                "long",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn mysql_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MySqlCdcSource".to_string(),
        display_name: "MySQL CDC Source (Debezium V1)".to_string(),
        connector_class: "MySqlCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from MySQL databases (V1)".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field(
                "database.server.name",
                "Logical name for the MySQL server",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.ssl.mode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disabled".to_string(),
                    "preferred".to_string(),
                    "required".to_string(),
                    "verify-ca".to_string(),
                    "verify-identity".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                    "schema_only".to_string(),
                ]),
            ),
            config_field(
                "binlog.buffer.size",
                "Size of the buffer used for binlog events",
                "int",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
            config_field(
                "max.queue.size",
                "Maximum number of records to queue",
                "int",
                false,
                None,
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "connect.timeout.ms",
                "Connection timeout in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "socket.timeout.ms",
                "Socket timeout in milliseconds",
                "long",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn mysql_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "MySQLSource".to_string(),
        display_name: "MySQL Source (JDBC)".to_string(),
        connector_class: "MySQLSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from MySQL databases".to_string(),
        required_configs: vec![
            config_field("connection.host", "Database hostname", "string", true, None),
            config_field("connection.port", "Database port", "int", true, None),
            config_field("connection.user", "Database username", "string", true, None),
            config_field("db.name", "Database name", "string", true, None),
            config_field(
                "table.whitelist",
                "Comma-separated list of tables to include",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "connection.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "connection.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "table.blacklist",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field("query", "Custom SQL query", "string", false, None),
            config_field(
                "incrementing.column.name",
                "Incrementing column name",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["connection.password".to_string()],
    }
}

pub(crate) fn oracle_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "OracleCdcSource".to_string(),
        display_name: "Oracle CDC Source".to_string(),
        connector_class: "OracleCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from Oracle databases".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field("database.dbname", "Database name", "string", true, None),
            config_field(
                "topic.prefix",
                "Topic prefix for CDC events",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "database.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                ]),
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn oracle_xstream_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "OracleXStreamCdcSource".to_string(),
        display_name: "Oracle XStream CDC Source".to_string(),
        connector_class: "OracleXStreamCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from Oracle databases using XStream".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field("database.dbname", "Database name", "string", true, None),
            config_field(
                "topic.prefix",
                "Topic prefix for CDC events",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "database.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                ]),
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
            config_field(
                "xstream.server.name",
                "XStream server name",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn oracle_database_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "OracleDatabaseSource".to_string(),
        display_name: "Oracle Database Source (JDBC)".to_string(),
        connector_class: "OracleDatabaseSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Oracle databases".to_string(),
        required_configs: vec![
            config_field("connection.host", "Database hostname", "string", true, None),
            config_field("connection.port", "Database port", "int", true, None),
            config_field("connection.user", "Database username", "string", true, None),
            config_field("db.name", "Database name", "string", true, None),
            config_field(
                "table.whitelist",
                "Comma-separated list of tables to include",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "connection.password",
                "Database password",
                "string",
                false,
                None,
            ),
            config_field(
                "connection.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                    "prefer".to_string(),
                ]),
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "table.blacklist",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field("query", "Custom SQL query", "string", false, None),
            config_field(
                "incrementing.column.name",
                "Incrementing column name",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["connection.password".to_string()],
    }
}

pub(crate) fn postgresql_cdc_source_v2() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "PostgresCdcSourceV2".to_string(),
        display_name: "PostgreSQL CDC Source V2 (Debezium)".to_string(),
        connector_class: "PostgresCdcSourceV2".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from PostgreSQL databases (V2)".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field("database.dbname", "Database name", "string", true, None),
            config_field(
                "topic.prefix",
                "Topic prefix for CDC events",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                ]),
            ),
            config_field(
                "publication.name",
                "PostgreSQL publication name",
                "string",
                false,
                None,
            ),
            config_field(
                "publication.autocreate.mode",
                "Publication auto-creation mode",
                "string",
                false,
                Some(vec![
                    "disabled".to_string(),
                    "all_tables".to_string(),
                    "filtered".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                ]),
            ),
            config_field(
                "slot.name",
                "PostgreSQL replication slot name",
                "string",
                false,
                None,
            ),
            config_field(
                "plugin.name",
                "PostgreSQL logical decoding plugin",
                "string",
                false,
                Some(vec!["pgoutput".to_string(), "wal2json".to_string()]),
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn postgresql_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "PostgresCdcSource".to_string(),
        display_name: "PostgreSQL CDC Source (Debezium V1)".to_string(),
        connector_class: "PostgresCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from PostgreSQL databases (V1)".to_string(),
        required_configs: vec![
            config_field(
                "database.hostname",
                "Database hostname",
                "string",
                true,
                None,
            ),
            config_field("database.port", "Database port", "int", true, None),
            config_field("database.user", "Database username", "string", true, None),
            config_field("database.dbname", "Database name", "string", true, None),
            config_field(
                "topic.prefix",
                "Topic prefix for CDC events",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "database.sslmode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                ]),
            ),
            config_field(
                "publication.name",
                "PostgreSQL publication name",
                "string",
                false,
                None,
            ),
            config_field(
                "publication.autocreate.mode",
                "Publication auto-creation mode",
                "string",
                false,
                Some(vec![
                    "disabled".to_string(),
                    "all_tables".to_string(),
                    "filtered".to_string(),
                ]),
            ),
            config_field(
                "snapshot.mode",
                "Snapshot mode for initial data capture",
                "string",
                false,
                Some(vec![
                    "initial".to_string(),
                    "never".to_string(),
                    "when_needed".to_string(),
                ]),
            ),
            config_field(
                "slot.name",
                "PostgreSQL replication slot name",
                "string",
                false,
                None,
            ),
            config_field(
                "plugin.name",
                "PostgreSQL logical decoding plugin",
                "string",
                false,
                Some(vec!["pgoutput".to_string(), "wal2json".to_string()]),
            ),
            config_field(
                "table.include.list",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.exclude.list",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.batch.size",
                "Maximum number of records in a single batch",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["database.password".to_string()],
    }
}

pub(crate) fn postgresql_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "PostgreSQLSource".to_string(),
        display_name: "PostgreSQL Source (JDBC)".to_string(),
        connector_class: "PostgreSQLSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from PostgreSQL databases".to_string(),
        required_configs: vec![
            config_field("connection.host", "Database hostname", "string", true, None),
            config_field("connection.port", "Database port", "int", true, None),
            config_field("connection.user", "Database username", "string", true, None),
            config_field("db.name", "Database name", "string", true, None),
        ],
        optional_configs: vec![
            config_field(
                "ssl.mode",
                "SSL mode for database connection",
                "string",
                false,
                Some(vec![
                    "disable".to_string(),
                    "prefer".to_string(),
                    "require".to_string(),
                    "verify-ca".to_string(),
                    "verify-full".to_string(),
                ]),
            ),
            config_field(
                "table.whitelist",
                "Comma-separated list of tables to include",
                "string",
                false,
                None,
            ),
            config_field(
                "table.blacklist",
                "Comma-separated list of tables to exclude",
                "string",
                false,
                None,
            ),
            config_field(
                "mode",
                "Incremental mode",
                "string",
                false,
                Some(vec![
                    "bulk".to_string(),
                    "timestamp".to_string(),
                    "incrementing".to_string(),
                ]),
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field("db.timezone", "Database timezone", "string", false, None),
            config_field(
                "table.types",
                "Table types to include",
                "string",
                false,
                Some(vec!["TABLE".to_string(), "VIEW".to_string()]),
            ),
            config_field("query", "Custom query to execute", "string", false, None),
        ],
        sensitive_configs: vec!["connection.password".to_string()],
    }
}

pub(crate) fn rabbitmq_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "RabbitMQSource".to_string(),
        display_name: "RabbitMQ Source".to_string(),
        connector_class: "RabbitMQSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read messages from RabbitMQ".to_string(),
        required_configs: vec![
            config_field("rabbitmq.host", "RabbitMQ hostname", "string", true, None),
            config_field("rabbitmq.port", "RabbitMQ port", "int", true, None),
            config_field(
                "rabbitmq.username",
                "RabbitMQ username",
                "string",
                true,
                None,
            ),
            config_field(
                "rabbitmq.queue.name",
                "RabbitMQ queue name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "rabbitmq.password",
                "RabbitMQ password",
                "string",
                false,
                None,
            ),
            config_field(
                "rabbitmq.virtual.host",
                "RabbitMQ virtual host",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "rabbitmq.exchange.name",
                "RabbitMQ exchange name",
                "string",
                false,
                None,
            ),
            config_field(
                "rabbitmq.routing.key",
                "RabbitMQ routing key",
                "string",
                false,
                None,
            ),
            config_field(
                "rabbitmq.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["rabbitmq.password".to_string()],
    }
}

pub(crate) fn salesforce_bulk_api_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SalesforceBulkAPISource".to_string(),
        display_name: "Salesforce Bulk API Source".to_string(),
        connector_class: "SalesforceBulkAPISource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Salesforce using Bulk API".to_string(),
        required_configs: vec![
            config_field(
                "salesforce.username",
                "Salesforce username",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.password",
                "Salesforce password",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.security.token",
                "Salesforce security token",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.sobject",
                "Salesforce SObject name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "salesforce.instance.url",
                "Salesforce instance URL",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "salesforce.query",
                "Salesforce SOQL query",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.bulk.api.version",
                "Salesforce Bulk API version",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.batch.size",
                "Salesforce batch size",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "salesforce.password".to_string(),
            "salesforce.security.token".to_string(),
        ],
    }
}

pub(crate) fn salesforce_bulk_api_2_0_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SalesforceBulkAPI2_0Source".to_string(),
        display_name: "Salesforce Bulk API 2.0 Source".to_string(),
        connector_class: "SalesforceBulkAPI2_0Source".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Salesforce using Bulk API 2.0".to_string(),
        required_configs: vec![
            config_field(
                "salesforce.username",
                "Salesforce username",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.password",
                "Salesforce password",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.security.token",
                "Salesforce security token",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.sobject",
                "Salesforce SObject name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "salesforce.instance.url",
                "Salesforce instance URL",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "salesforce.query",
                "Salesforce SOQL query",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.bulk.api.version",
                "Salesforce Bulk API version",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.batch.size",
                "Salesforce batch size",
                "int",
                false,
                None,
            ),
            config_field(
                "salesforce.bulk.api.2.0.enabled",
                "Enable Bulk API 2.0",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec![
            "salesforce.password".to_string(),
            "salesforce.security.token".to_string(),
        ],
    }
}

pub(crate) fn salesforce_cdc_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SalesforceCdcSource".to_string(),
        display_name: "Salesforce CDC Source".to_string(),
        connector_class: "SalesforceCdcSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Capture change data from Salesforce".to_string(),
        required_configs: vec![
            config_field(
                "salesforce.username",
                "Salesforce username",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.password",
                "Salesforce password",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.security.token",
                "Salesforce security token",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.sobject",
                "Salesforce SObject name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "salesforce.instance.url",
                "Salesforce instance URL",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "salesforce.query",
                "Salesforce SOQL query",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.cdc.enabled",
                "Enable CDC",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
            config_field(
                "salesforce.cdc.topic",
                "Salesforce CDC topic",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "salesforce.password".to_string(),
            "salesforce.security.token".to_string(),
        ],
    }
}

pub(crate) fn salesforce_platform_event_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SalesforcePlatformEventSource".to_string(),
        display_name: "Salesforce Platform Event Source".to_string(),
        connector_class: "SalesforcePlatformEventSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read Salesforce Platform Events".to_string(),
        required_configs: vec![
            config_field(
                "salesforce.username",
                "Salesforce username",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.password",
                "Salesforce password",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.security.token",
                "Salesforce security token",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.platform.event.name",
                "Salesforce Platform Event name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "salesforce.instance.url",
                "Salesforce instance URL",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "salesforce.platform.event.replay.id",
                "Salesforce Platform Event replay ID",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.platform.event.replay.preset",
                "Salesforce Platform Event replay preset",
                "string",
                false,
                Some(vec![
                    "ALL_TIME".to_string(),
                    "LAST_24_HOURS".to_string(),
                    "LAST_7_DAYS".to_string(),
                    "LAST_30_DAYS".to_string(),
                ]),
            ),
        ],
        sensitive_configs: vec![
            "salesforce.password".to_string(),
            "salesforce.security.token".to_string(),
        ],
    }
}

pub(crate) fn salesforce_pushtopic_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SalesforcePushTopicSource".to_string(),
        display_name: "Salesforce PushTopic Source".to_string(),
        connector_class: "SalesforcePushTopicSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read Salesforce PushTopics".to_string(),
        required_configs: vec![
            config_field(
                "salesforce.username",
                "Salesforce username",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.password",
                "Salesforce password",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.security.token",
                "Salesforce security token",
                "string",
                true,
                None,
            ),
            config_field(
                "salesforce.pushtopic.name",
                "Salesforce PushTopic name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "salesforce.instance.url",
                "Salesforce instance URL",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "salesforce.pushtopic.query",
                "Salesforce PushTopic query",
                "string",
                false,
                None,
            ),
            config_field(
                "salesforce.pushtopic.notify.for.fields",
                "Salesforce PushTopic notify for fields",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "salesforce.password".to_string(),
            "salesforce.security.token".to_string(),
        ],
    }
}

pub(crate) fn servicenow_source_v2() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "ServiceNowSourceV2".to_string(),
        display_name: "ServiceNow Source V2".to_string(),
        connector_class: "ServiceNowSourceV2".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from ServiceNow (V2)".to_string(),
        required_configs: vec![
            config_field("servicenow.url", "ServiceNow URL", "string", true, None),
            config_field(
                "servicenow.username",
                "ServiceNow username",
                "string",
                true,
                None,
            ),
            config_field(
                "servicenow.table.name",
                "ServiceNow table name",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "servicenow.password",
                "ServiceNow password",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "servicenow.query",
                "ServiceNow query",
                "string",
                false,
                None,
            ),
            config_field(
                "servicenow.fields",
                "ServiceNow fields",
                "string",
                false,
                None,
            ),
            config_field(
                "servicenow.sysparm.limit",
                "ServiceNow sysparm limit",
                "int",
                false,
                None,
            ),
        ],
        sensitive_configs: vec!["servicenow.password".to_string()],
    }
}

pub(crate) fn sftp_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SFTPSource".to_string(),
        display_name: "SFTP Source".to_string(),
        connector_class: "SFTPSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read files from SFTP servers".to_string(),
        required_configs: vec![
            config_field("sftp.host", "SFTP hostname", "string", true, None),
            config_field("sftp.port", "SFTP port", "int", true, None),
            config_field("sftp.username", "SFTP username", "string", true, None),
            config_field("sftp.remote.path", "SFTP remote path", "string", true, None),
        ],
        optional_configs: vec![
            config_field("sftp.password", "SFTP password", "string", false, None),
            config_field(
                "sftp.private.key",
                "SFTP private key",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field(
                "sftp.file.pattern",
                "SFTP file pattern",
                "string",
                false,
                None,
            ),
            config_field(
                "sftp.file.encoding",
                "SFTP file encoding",
                "string",
                false,
                None,
            ),
            config_field(
                "sftp.ssl.enabled",
                "Enable SSL",
                "boolean",
                false,
                Some(vec!["true".to_string(), "false".to_string()]),
            ),
        ],
        sensitive_configs: vec!["sftp.password".to_string(), "sftp.private.key".to_string()],
    }
}

pub(crate) fn snowflake_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "SnowflakeSource".to_string(),
        display_name: "Snowflake Source".to_string(),
        connector_class: "SnowflakeSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Snowflake".to_string(),
        required_configs: vec![
            config_field("snowflake.url", "Snowflake URL", "string", true, None),
            config_field(
                "snowflake.username",
                "Snowflake username",
                "string",
                true,
                None,
            ),
            config_field(
                "snowflake.database",
                "Snowflake database",
                "string",
                true,
                None,
            ),
            config_field("snowflake.schema", "Snowflake schema", "string", true, None),
            config_field("snowflake.table", "Snowflake table", "string", true, None),
        ],
        optional_configs: vec![
            config_field(
                "snowflake.password",
                "Snowflake password",
                "string",
                false,
                None,
            ),
            config_field(
                "snowflake.private.key",
                "Snowflake private key",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("snowflake.query", "Snowflake query", "string", false, None),
            config_field(
                "snowflake.warehouse",
                "Snowflake warehouse",
                "string",
                false,
                None,
            ),
            config_field("snowflake.role", "Snowflake role", "string", false, None),
        ],
        sensitive_configs: vec![
            "snowflake.password".to_string(),
            "snowflake.private.key".to_string(),
        ],
    }
}

pub(crate) fn zendesk_source() -> ConnectorDefinition {
    ConnectorDefinition {
        name: "ZendeskSource".to_string(),
        display_name: "Zendesk Source".to_string(),
        connector_class: "ZendeskSource".to_string(),
        connector_type: ConnectorType::Source,
        description: "Read data from Zendesk".to_string(),
        required_configs: vec![
            config_field("zendesk.url", "Zendesk URL", "string", true, None),
            config_field("zendesk.username", "Zendesk username", "string", true, None),
            config_field(
                "zendesk.object.type",
                "Zendesk object type",
                "string",
                true,
                None,
            ),
        ],
        optional_configs: vec![
            config_field(
                "zendesk.password",
                "Zendesk password",
                "string",
                false,
                None,
            ),
            config_field(
                "zendesk.api.token",
                "Zendesk API token",
                "string",
                false,
                None,
            ),
            config_field(
                "poll.interval.ms",
                "Polling interval in milliseconds",
                "long",
                false,
                None,
            ),
            config_field(
                "max.records",
                "Maximum number of records to fetch",
                "int",
                false,
                None,
            ),
            config_field("zendesk.query", "Zendesk query", "string", false, None),
            config_field("zendesk.fields", "Zendesk fields", "string", false, None),
            config_field(
                "zendesk.sort.order",
                "Zendesk sort order",
                "string",
                false,
                None,
            ),
        ],
        sensitive_configs: vec![
            "zendesk.password".to_string(),
            "zendesk.api.token".to_string(),
        ],
    }
}
