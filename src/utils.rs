// ABOUTME: Utility functions for validation and error handling
// ABOUTME: Provides input validation, retry logic, and resource cleanup

use anyhow::{bail, Result};
use std::time::Duration;
use which::which;

/// Validate a PostgreSQL connection string
pub fn validate_connection_string(url: &str) -> Result<()> {
    if url.trim().is_empty() {
        bail!("Connection string cannot be empty");
    }

    // Check for common URL schemes
    if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
        bail!(
            "Invalid connection string format.\n\
             Expected format: postgresql://user:password@host:port/database\n\
             Got: {}",
            url
        );
    }

    // Check for minimum required components (user@host/database)
    if !url.contains('@') {
        bail!(
            "Connection string missing user credentials.\n\
             Expected format: postgresql://user:password@host:port/database"
        );
    }

    if !url.contains('/') || url.matches('/').count() < 3 {
        bail!(
            "Connection string missing database name.\n\
             Expected format: postgresql://user:password@host:port/database"
        );
    }

    Ok(())
}

/// Check that required PostgreSQL client tools are available
pub fn check_required_tools() -> Result<()> {
    let tools = ["pg_dump", "pg_dumpall", "psql"];
    let mut missing = Vec::new();

    for tool in &tools {
        if which(tool).is_err() {
            missing.push(*tool);
        }
    }

    if !missing.is_empty() {
        bail!(
            "Missing required PostgreSQL client tools: {}\n\
             \n\
             Please install PostgreSQL client tools:\n\
             - Ubuntu/Debian: sudo apt-get install postgresql-client\n\
             - macOS: brew install postgresql\n\
             - RHEL/CentOS: sudo yum install postgresql\n\
             - Windows: Download from https://www.postgresql.org/download/windows/",
            missing.join(", ")
        );
    }

    Ok(())
}

/// Retry a function with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut delay = initial_delay;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);

                if attempt < max_retries {
                    tracing::warn!(
                        "Operation failed (attempt {}/{}), retrying in {:?}...",
                        attempt + 1,
                        max_retries + 1,
                        delay
                    );
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Operation failed after retries")))
}

/// Sanitize an identifier (table name, schema name, etc.) for display
/// This doesn't make it safe for SQL (we use parameterized queries for that)
/// but ensures it's safe to display in error messages
pub fn sanitize_identifier(identifier: &str) -> String {
    // Remove any control characters and limit length for display
    identifier
        .chars()
        .filter(|c| !c.is_control())
        .take(100)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_connection_string_valid() {
        assert!(validate_connection_string("postgresql://user:pass@localhost:5432/dbname").is_ok());
        assert!(validate_connection_string("postgres://user@host/db").is_ok());
    }

    #[test]
    fn test_check_required_tools() {
        // This test will pass if PostgreSQL client tools are installed
        // It will fail (appropriately) if they're not installed
        let result = check_required_tools();

        // On systems with PostgreSQL installed, this should pass
        // On systems without it, we expect a specific error message
        if result.is_err() {
            let err_msg = result.unwrap_err().to_string();
            assert!(err_msg.contains("Missing required PostgreSQL client tools"));
            assert!(err_msg.contains("pg_dump") || err_msg.contains("pg_dumpall") || err_msg.contains("psql"));
        }
    }

    #[test]
    fn test_validate_connection_string_invalid() {
        assert!(validate_connection_string("").is_err());
        assert!(validate_connection_string("   ").is_err());
        assert!(validate_connection_string("mysql://localhost/db").is_err());
        assert!(validate_connection_string("postgresql://localhost").is_err());
        assert!(validate_connection_string("postgresql://localhost/db").is_err());
        // Missing user
    }

    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("normal_table"), "normal_table");
        assert_eq!(sanitize_identifier("table\x00name"), "tablename");
        assert_eq!(sanitize_identifier("table\nname"), "tablename");

        // Test length limit
        let long_name = "a".repeat(200);
        assert_eq!(sanitize_identifier(&long_name).len(), 100);
    }

    #[tokio::test]
    async fn test_retry_with_backoff_success() {
        let mut attempts = 0;
        let result = retry_with_backoff(
            || {
                attempts += 1;
                async move {
                    if attempts < 3 {
                        anyhow::bail!("Temporary failure")
                    } else {
                        Ok("Success")
                    }
                }
            },
            5,
            Duration::from_millis(10),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(attempts, 3);
    }

    #[tokio::test]
    async fn test_retry_with_backoff_failure() {
        let mut attempts = 0;
        let result: Result<&str> = retry_with_backoff(
            || {
                attempts += 1;
                async move { anyhow::bail!("Permanent failure") }
            },
            2,
            Duration::from_millis(10),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempts, 3); // Initial + 2 retries
    }
}
