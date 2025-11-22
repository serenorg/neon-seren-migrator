# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2.5.0

### Added

- **Remote Execution (AWS)**: SerenAI-managed cloud service for running replication jobs
  - Remote-by-default execution mode with `--local` flag for local fallback
  - Job submission API with encrypted credentials via AWS KMS
  - Status polling and real-time progress monitoring
  - Automatic EC2 instance provisioning and termination
  - Integration tests for remote execution functionality
- **Job Spec Validation**: Comprehensive API validation framework
  - Schema versioning (current: v1.0) with backward compatibility
  - PostgreSQL URL security validation with injection prevention
  - Required field validation, command whitelist, and size limits (15KB max)
  - Test suite with 18 validation tests
  - API schema documentation at `docs/api-schema.md`
- **Observability**: Built-in monitoring and tracing
  - Trace ID generation for request tracking across systems
  - CloudWatch Logs integration with structured logging
  - CloudWatch Metrics for job lifecycle events
  - Log URLs returned in API responses for troubleshooting
- **CI/CD Improvements**: Enhanced testing and deployment
  - Smoke tests for AWS infrastructure validation
  - Environment-specific configurations (dev, staging, prod)
  - Comprehensive CI/CD documentation at `docs/cicd.md`
  - Automated release workflows
- **Security Features**: Enterprise-grade security controls
  - KMS encryption for database credentials at rest in DynamoDB
  - Credential redaction in all logs and outputs
  - IAM role-based access with least-privilege policies
  - API key authentication via SSM Parameter Store
  - Security model documentation at `docs/aws-setup.md`
- **Reliability Controls**: Production-ready resilience
  - Job timeout controls (default: 8 hours, configurable)
  - Maximum instance runtime limits to prevent runaway costs
  - Graceful error handling with detailed error messages
  - Connection retry with exponential backoff
- **Documentation**: Comprehensive user and developer guides
  - Remote execution guide in README with usage examples
  - AWS setup guide (23KB) for infrastructure deployment
  - API schema specification with migration guidance
  - CI/CD pipeline documentation
  - SerenDB signup instructions (optional target database)

### Changed

- `init` command now uses remote execution by default (use `--local` to run locally)
- Job specifications now require `schema_version` field (current: "1.0")
- API endpoint standardized to `https://api.seren.cloud/replication`

### Improved

- Error messages now include trace IDs for support ticket correlation
- Job submission failures provide clear fallback instructions (`--local` flag)
- CloudWatch integration enables post-mortem debugging of failed jobs
- Cost management with automatic instance termination after completion

## [1.2.0] - 2025-11-07
### Added
- Table-level replication rules with new CLI flags (`--schema-only-tables`, `--table-filter`, `--time-filter`) and TOML config support (`--config`).
- Filtered snapshot pipeline that streams predicate-matching rows and skips schema-only tables during `init`.
- Predicate-aware publications for `sync`, enabling logical replication that respects table filters on PostgreSQL 15+.
- TimescaleDB-focused documentation at `docs/replication-config.md` plus expanded README guidance.

### Improved
- Multi-database `init` now checkpoints per-table progress and resumes from the last successful database.

## [1.1.1] - 2025-11-07
- Previous improvements and fixes bundled with v1.1.1.
