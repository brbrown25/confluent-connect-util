use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectUtilError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Terraform generation error: {0}")]
    Terraform(String),

    #[error("User input error: {0}")]
    UserInput(String),

    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for ConnectUtilError {
    fn from(err: anyhow::Error) -> Self {
        ConnectUtilError::Unknown(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_config_error() {
        let error = ConnectUtilError::Config("test config error".to_string());
        assert!(error.to_string().contains("Configuration error"));
        assert!(error.to_string().contains("test config error"));
    }

    #[test]
    fn test_validation_error() {
        let error = ConnectUtilError::Validation("test validation error".to_string());
        assert!(error.to_string().contains("Validation error"));
        assert!(error.to_string().contains("test validation error"));
    }

    #[test]
    fn test_terraform_error() {
        let error = ConnectUtilError::Terraform("test terraform error".to_string());
        assert!(error.to_string().contains("Terraform generation error"));
        assert!(error.to_string().contains("test terraform error"));
    }

    #[test]
    fn test_user_input_error() {
        let error = ConnectUtilError::UserInput("test user input error".to_string());
        assert!(error.to_string().contains("User input error"));
        assert!(error.to_string().contains("test user input error"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = IoError::new(ErrorKind::NotFound, "file not found");
        let connect_error: ConnectUtilError = io_error.into();

        match connect_error {
            ConnectUtilError::Io(err) => {
                assert_eq!(err.kind(), ErrorKind::NotFound);
                assert!(err.to_string().contains("file not found"));
            }
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error =
            serde_json::Error::io(IoError::new(ErrorKind::InvalidData, "invalid json"));
        let connect_error: ConnectUtilError = json_error.into();

        match connect_error {
            ConnectUtilError::Json(err) => {
                assert!(err.to_string().contains("invalid json"));
            }
            _ => panic!("Expected Json error variant"),
        }
    }

    #[test]
    fn test_anyhow_error_conversion() {
        let anyhow_error = anyhow::Error::msg("anyhow error message");
        let connect_error: ConnectUtilError = anyhow_error.into();

        match connect_error {
            ConnectUtilError::Unknown(msg) => {
                assert!(msg.contains("anyhow error message"));
            }
            _ => panic!("Expected Unknown error variant"),
        }
    }

    #[test]
    fn test_unknown_error() {
        let error = ConnectUtilError::Unknown("test unknown error".to_string());
        assert!(error.to_string().contains("Unknown error"));
        assert!(error.to_string().contains("test unknown error"));
    }

    #[test]
    fn test_error_debug_trait() {
        let error = ConnectUtilError::Config("test error".to_string());
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("Config"));
        assert!(debug_string.contains("test error"));
    }

    #[test]
    fn test_error_display_trait() {
        let error = ConnectUtilError::Validation("validation failed".to_string());
        let display_string = format!("{}", error);
        assert!(display_string.contains("Validation error"));
        assert!(display_string.contains("validation failed"));
    }
}
