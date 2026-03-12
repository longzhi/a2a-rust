use serde::{Deserialize, Serialize};

use crate::types::JsonObject;

use super::message::{Artifact, Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub context_id: String,
    pub status: TaskStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatus {
    pub state: TaskState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    #[default]
    #[serde(rename = "TASK_STATE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "TASK_STATE_SUBMITTED")]
    Submitted,
    #[serde(rename = "TASK_STATE_WORKING")]
    Working,
    #[serde(rename = "TASK_STATE_COMPLETED")]
    Completed,
    #[serde(rename = "TASK_STATE_FAILED")]
    Failed,
    #[serde(rename = "TASK_STATE_CANCELED")]
    Canceled,
    #[serde(rename = "TASK_STATE_INPUT_REQUIRED")]
    InputRequired,
    #[serde(rename = "TASK_STATE_REJECTED")]
    Rejected,
    #[serde(rename = "TASK_STATE_AUTH_REQUIRED")]
    AuthRequired,
}

#[cfg(test)]
mod tests {
    use super::{Task, TaskState, TaskStatus};
    use crate::types::{Message, Part, Role};

    #[test]
    fn task_state_serializes_as_proto_enum_name() {
        let json =
            serde_json::to_string(&TaskState::Completed).expect("task state should serialize");
        assert_eq!(json, r#""TASK_STATE_COMPLETED""#);
    }

    #[test]
    fn task_round_trip_serialization() {
        let task = Task {
            id: "task-1".to_owned(),
            context_id: "ctx-1".to_owned(),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    message_id: "msg-1".to_owned(),
                    context_id: Some("ctx-1".to_owned()),
                    task_id: Some("task-1".to_owned()),
                    role: Role::Agent,
                    parts: vec![Part {
                        text: Some("done".to_owned()),
                        raw: None,
                        url: None,
                        data: None,
                        metadata: None,
                        filename: None,
                        media_type: None,
                    }],
                    metadata: None,
                    extensions: Vec::new(),
                    reference_task_ids: Vec::new(),
                }),
                timestamp: Some("2026-03-12T12:00:00Z".to_owned()),
            },
            artifacts: Vec::new(),
            history: Vec::new(),
            metadata: None,
        };

        let json = serde_json::to_string(&task).expect("task should serialize");
        let round_trip: Task = serde_json::from_str(&json).expect("task should deserialize");

        assert_eq!(round_trip.id, "task-1");
        assert_eq!(round_trip.context_id, "ctx-1");
        assert_eq!(round_trip.status.state, TaskState::Completed);
    }
}
