// ABOUTME: Replication utilities module
// ABOUTME: Handles PostgreSQL logical replication setup and monitoring

pub mod publication;
pub mod subscription;
pub mod monitor;

pub use publication::{create_publication, list_publications, drop_publication};
pub use subscription::{create_subscription, list_subscriptions, drop_subscription, wait_for_sync};
pub use monitor::{
    get_replication_lag, get_subscription_status, is_replication_caught_up,
    SourceReplicationStats, SubscriptionStats,
};
