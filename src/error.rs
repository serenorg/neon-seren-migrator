// ABOUTME: Custom error types for the migrator
// ABOUTME: Provides context-specific error variants with actionable messages

use std::fmt;

#[derive(Debug)]
pub enum MigratorError {
    Connection(String),
    Permission(String),
    Validation(String),
    Migration(String),
}

impl fmt::Display for MigratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MigratorError::Connection(msg) => write!(f, "Connection error: {}", msg),
            MigratorError::Permission(msg) => write!(f, "Permission error: {}", msg),
            MigratorError::Validation(msg) => write!(f, "Validation error: {}", msg),
            MigratorError::Migration(msg) => write!(f, "Migration error: {}", msg),
        }
    }
}

impl std::error::Error for MigratorError {}
