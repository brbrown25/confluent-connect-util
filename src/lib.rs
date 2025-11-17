pub mod app;
pub mod connectors;
pub mod error;
pub mod terraform;
pub mod types;

pub use error::ConnectUtilError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ConnectUtilError::Config("test error".to_string());
        assert!(error.to_string().contains("test error"));
    }
}
