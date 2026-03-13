use serde::{Deserialize, Serialize};

/// Push-notification delivery target.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional provider-specific configuration identifier.
    pub id: Option<String>,
    /// Destination URL for push delivery.
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional opaque bearer token or shared secret.
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional authentication description.
    pub authentication: Option<AuthenticationInfo>,
}

/// Authentication details for a push target.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationInfo {
    /// Authentication scheme name.
    pub scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional scheme-specific credentials.
    pub credentials: Option<String>,
}

/// Stored push-notification configuration associated with a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskPushNotificationConfig {
    /// Unique configuration identifier.
    pub id: String,
    /// Task identifier that owns the configuration.
    pub task_id: String,
    /// Push delivery settings.
    pub push_notification_config: PushNotificationConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional tenant associated with the configuration.
    pub tenant: Option<String>,
}
