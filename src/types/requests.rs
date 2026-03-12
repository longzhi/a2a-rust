use serde::{Deserialize, Serialize};

use crate::A2AError;
use crate::types::JsonObject;

use super::message::Message;
use super::push::PushNotificationConfig;
use super::task::TaskState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_output_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_notification_config: Option<PushNotificationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_length: Option<i32>,
    #[serde(default, skip_serializing_if = "crate::types::is_false")]
    pub blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    pub message: Message,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<SendMessageConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

impl SendMessageRequest {
    pub fn validate(&self) -> Result<(), A2AError> {
        self.message.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskRequest {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTasksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_timestamp_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_artifacts: Option<bool>,
}

impl ListTasksRequest {
    pub fn validate(&self) -> Result<(), A2AError> {
        if let Some(page_size) = self.page_size
            && !(1..=100).contains(&page_size)
        {
            return Err(A2AError::InvalidRequest(
                "pageSize must be between 1 and 100".to_owned(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTaskRequest {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskPushNotificationConfigRequest {
    pub id: String,
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskPushNotificationConfigRequest {
    pub id: String,
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskPushNotificationConfigRequest {
    pub task_id: String,
    pub config_id: String,
    pub config: PushNotificationConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeToTaskRequest {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTaskPushNotificationConfigRequest {
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

impl ListTaskPushNotificationConfigRequest {
    pub fn validate(&self) -> Result<(), A2AError> {
        if self.task_id.is_empty() {
            return Err(A2AError::InvalidRequest(
                "task_id must not be empty".to_owned(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ListTaskPushNotificationConfigRequest, ListTasksRequest, SendMessageRequest};
    use crate::types::{Message, Part, Role};

    #[test]
    fn list_task_push_notification_config_request_rejects_empty_task_id() {
        let request = ListTaskPushNotificationConfigRequest {
            task_id: String::new(),
            page_size: None,
            page_token: None,
            tenant: None,
        };

        let error = request.validate().expect_err("request should be invalid");
        assert!(error.to_string().contains("task_id must not be empty"));
    }

    #[test]
    fn list_tasks_request_rejects_out_of_range_page_size() {
        let request = ListTasksRequest {
            tenant: None,
            context_id: None,
            status: None,
            page_size: Some(101),
            page_token: None,
            history_length: None,
            status_timestamp_after: None,
            include_artifacts: None,
        };

        let error = request.validate().expect_err("request should be invalid");
        assert!(
            error
                .to_string()
                .contains("pageSize must be between 1 and 100")
        );
    }

    #[test]
    fn send_message_request_rejects_empty_message_parts() {
        let request = SendMessageRequest {
            message: Message {
                message_id: "msg-1".to_owned(),
                context_id: None,
                task_id: None,
                role: Role::User,
                parts: Vec::new(),
                metadata: None,
                extensions: Vec::new(),
                reference_task_ids: Vec::new(),
            },
            configuration: None,
            metadata: None,
            tenant: None,
        };

        let error = request.validate().expect_err("request should be invalid");
        assert!(
            error
                .to_string()
                .contains("message must contain at least one part")
        );
    }

    #[test]
    fn send_message_request_validates_part_content() {
        let request = SendMessageRequest {
            message: Message {
                message_id: "msg-1".to_owned(),
                context_id: None,
                task_id: None,
                role: Role::User,
                parts: vec![Part {
                    text: Some("hello".to_owned()),
                    raw: Some(vec![104, 105]),
                    url: None,
                    data: None,
                    metadata: None,
                    filename: None,
                    media_type: None,
                }],
                metadata: None,
                extensions: Vec::new(),
                reference_task_ids: Vec::new(),
            },
            configuration: None,
            metadata: None,
            tenant: None,
        };

        let error = request.validate().expect_err("request should be invalid");
        assert!(
            error
                .to_string()
                .contains("part cannot contain more than one")
        );
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExtendedAgentCardRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}
