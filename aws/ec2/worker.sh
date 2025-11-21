#!/bin/bash
# ABOUTME: EC2 worker bootstrap script for remote replication jobs
# ABOUTME: Executes replication job and manages lifecycle from provisioning to completion

set -euo pipefail

# Configuration
REPLICATOR_BIN="/opt/seren-replicator/postgres-seren-replicator"
DYNAMODB_TABLE="${DYNAMODB_TABLE:-replication-jobs}"
AWS_REGION="${AWS_REGION:-us-east-1}"

# Parse arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <job_id> <job_spec_json_file>"
    exit 1
fi

JOB_ID="$1"
JOB_SPEC_FILE="$2"

# Log function
log() {
    echo "[$(date -u +"%Y-%m-%dT%H:%M:%SZ")] $*"
}

# Update DynamoDB job status
update_job_status() {
    local status="$1"
    local error_msg="${2:-}"

    log "Updating job status to: $status"

    if [ -n "$error_msg" ]; then
        aws dynamodb update-item \
            --region "$AWS_REGION" \
            --table-name "$DYNAMODB_TABLE" \
            --key "{\"job_id\": {\"S\": \"$JOB_ID\"}}" \
            --update-expression "SET #status = :status, error = :error" \
            --expression-attribute-names '{"#status": "status"}' \
            --expression-attribute-values "{\":status\": {\"S\": \"$status\"}, \":error\": {\"S\": \"$error_msg\"}}"
    else
        aws dynamodb update-item \
            --region "$AWS_REGION" \
            --table-name "$DYNAMODB_TABLE" \
            --key "{\"job_id\": {\"S\": \"$JOB_ID\"}}" \
            --update-expression "SET #status = :status, ${status}_at = :timestamp" \
            --expression-attribute-names '{"#status": "status"}' \
            --expression-attribute-values "{\":status\": {\"S\": \"$status\"}, \":timestamp\": {\"S\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}}"
    fi
}

# Update progress in DynamoDB
update_progress() {
    local current_db="$1"
    local completed="$2"
    local total="$3"

    local progress_json="{\"current_database\": \"$current_db\", \"databases_completed\": $completed, \"databases_total\": $total}"

    aws dynamodb update-item \
        --region "$AWS_REGION" \
        --table-name "$DYNAMODB_TABLE" \
        --key "{\"job_id\": {\"S\": \"$JOB_ID\"}}" \
        --update-expression "SET progress = :progress" \
        --expression-attribute-values "{\":progress\": {\"S\": \"$progress_json\"}}"
}

# Terminate this instance
terminate_self() {
    log "Self-terminating instance..."

    # Get instance ID from metadata service
    INSTANCE_ID=$(ec2-metadata --instance-id | cut -d " " -f 2)

    if [ -n "$INSTANCE_ID" ]; then
        aws ec2 terminate-instances \
            --region "$AWS_REGION" \
            --instance-ids "$INSTANCE_ID"
        log "Termination initiated for instance $INSTANCE_ID"
    else
        log "ERROR: Could not determine instance ID from metadata"
    fi
}

# Trap errors and update status
trap 'update_job_status "failed" "Script error at line $LINENO"; terminate_self' ERR

# Main execution
main() {
    log "Starting replication job: $JOB_ID"
    log "Job spec file: $JOB_SPEC_FILE"

    # Verify job spec file exists
    if [ ! -f "$JOB_SPEC_FILE" ]; then
        log "ERROR: Job spec file not found: $JOB_SPEC_FILE"
        update_job_status "failed" "Job spec file not found"
        terminate_self
        exit 1
    fi

    # Parse job specification
    log "Parsing job specification..."
    COMMAND=$(jq -r '.command' "$JOB_SPEC_FILE")
    SOURCE_URL=$(jq -r '.source_url' "$JOB_SPEC_FILE")
    TARGET_URL=$(jq -r '.target_url' "$JOB_SPEC_FILE")

    log "Command: $COMMAND"
    log "Source: ${SOURCE_URL%%@*}@***"  # Hide credentials in logs
    log "Target: ${TARGET_URL%%@*}@***"  # Hide credentials in logs

    # Update status to running
    update_job_status "running"

    # Build replicator command
    CMD=("$REPLICATOR_BIN" "$COMMAND" "--source" "$SOURCE_URL" "--target" "$TARGET_URL" "--yes")

    # Add filter options
    INCLUDE_DATABASES=$(jq -r '.filter.include_databases // empty | .[]' "$JOB_SPEC_FILE")
    if [ -n "$INCLUDE_DATABASES" ]; then
        while IFS= read -r db; do
            CMD+=("--include-databases" "$db")
        done <<< "$INCLUDE_DATABASES"
    fi

    EXCLUDE_TABLES=$(jq -r '.filter.exclude_tables // empty | .[]' "$JOB_SPEC_FILE")
    if [ -n "$EXCLUDE_TABLES" ]; then
        while IFS= read -r table; do
            CMD+=("--exclude-tables" "$table")
        done <<< "$EXCLUDE_TABLES"
    fi

    # Add options from job spec
    DROP_EXISTING=$(jq -r '.options.drop_existing // "false"' "$JOB_SPEC_FILE")
    if [ "$DROP_EXISTING" = "true" ]; then
        CMD+=("--drop-existing")
    fi

    NO_SYNC=$(jq -r '.options.no_sync // "false"' "$JOB_SPEC_FILE")
    if [ "$NO_SYNC" = "true" ]; then
        CMD+=("--no-sync")
    fi

    # Execute replication
    log "Executing replication command..."
    log "Command: ${CMD[*]}"

    if "${CMD[@]}"; then
        log "Replication completed successfully"
        update_job_status "completed"
    else
        EXIT_CODE=$?
        log "Replication failed with exit code: $EXIT_CODE"
        update_job_status "failed" "Replication command failed with exit code $EXIT_CODE"
    fi

    # Self-terminate
    terminate_self
}

# Run main function
main "$@"
