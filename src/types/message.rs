use serde::{Deserialize, Serialize};

use crate::A2AError;
use crate::types::JsonObject;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Role {
    #[default]
    #[serde(rename = "ROLE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "ROLE_USER")]
    User,
    #[serde(rename = "ROLE_AGENT")]
    Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::types::base64_bytes::option"
    )]
    pub raw: Option<Vec<u8>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

impl Part {
    pub fn content_count(&self) -> usize {
        usize::from(self.text.is_some())
            + usize::from(self.raw.is_some())
            + usize::from(self.url.is_some())
            + usize::from(self.data.is_some())
    }

    pub fn has_single_content(&self) -> bool {
        self.content_count() == 1
    }

    pub fn validate(&self) -> Result<(), A2AError> {
        match self.content_count() {
            1 => Ok(()),
            0 => Err(A2AError::InvalidRequest(
                "part must contain exactly one of text, raw, url, or data".to_owned(),
            )),
            _ => Err(A2AError::InvalidRequest(
                "part cannot contain more than one of text, raw, url, or data".to_owned(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    pub role: Role,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_task_ids: Vec<String>,
}

impl Message {
    pub fn validate(&self) -> Result<(), A2AError> {
        if self.parts.is_empty() {
            return Err(A2AError::InvalidRequest(
                "message must contain at least one part".to_owned(),
            ));
        }

        for part in &self.parts {
            part.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub artifact_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<Part>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonObject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<String>,
}

impl Artifact {
    pub fn validate(&self) -> Result<(), A2AError> {
        if self.parts.is_empty() {
            return Err(A2AError::InvalidRequest(
                "artifact must contain at least one part".to_owned(),
            ));
        }

        for part in &self.parts {
            part.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Artifact, Message, Part, Role};

    #[test]
    fn part_reports_single_content_field() {
        let part = Part {
            text: Some("hello".to_owned()),
            raw: None,
            url: None,
            data: None,
            metadata: None,
            filename: None,
            media_type: None,
        };

        assert_eq!(part.content_count(), 1);
        assert!(part.has_single_content());
    }

    #[test]
    fn part_raw_serializes_as_base64() {
        let part = Part {
            text: None,
            raw: Some(vec![104, 105]),
            url: None,
            data: None,
            metadata: None,
            filename: None,
            media_type: None,
        };

        let json = serde_json::to_string(&part).expect("part should serialize");
        assert_eq!(json, r#"{"raw":"aGk="}"#);
    }

    #[test]
    fn part_validate_rejects_multiple_content_fields() {
        let part = Part {
            text: Some("hello".to_owned()),
            raw: Some(vec![104, 105]),
            url: None,
            data: None,
            metadata: None,
            filename: None,
            media_type: None,
        };

        let error = part.validate().expect_err("part should be invalid");
        assert!(
            error
                .to_string()
                .contains("part cannot contain more than one")
        );
    }

    #[test]
    fn message_and_artifact_round_trip_serialization() {
        let message = Message {
            message_id: "msg-1".to_owned(),
            context_id: Some("ctx-1".to_owned()),
            task_id: Some("task-1".to_owned()),
            role: Role::User,
            parts: vec![Part {
                text: Some("hello".to_owned()),
                raw: None,
                url: None,
                data: None,
                metadata: None,
                filename: None,
                media_type: None,
            }],
            metadata: None,
            extensions: vec!["trace".to_owned()],
            reference_task_ids: vec!["task-0".to_owned()],
        };
        let artifact = Artifact {
            artifact_id: "artifact-1".to_owned(),
            name: Some("transcript".to_owned()),
            description: Some("conversation log".to_owned()),
            parts: vec![Part {
                text: Some("hello".to_owned()),
                raw: None,
                url: None,
                data: None,
                metadata: None,
                filename: None,
                media_type: None,
            }],
            metadata: None,
            extensions: vec!["indexed".to_owned()],
        };

        let message_json = serde_json::to_string(&message).expect("message should serialize");
        let artifact_json = serde_json::to_string(&artifact).expect("artifact should serialize");

        let message_round_trip: Message =
            serde_json::from_str(&message_json).expect("message should deserialize");
        let artifact_round_trip: Artifact =
            serde_json::from_str(&artifact_json).expect("artifact should deserialize");

        assert_eq!(message_round_trip.message_id, "msg-1");
        assert_eq!(artifact_round_trip.artifact_id, "artifact-1");
        assert_eq!(artifact_round_trip.parts.len(), 1);
    }

    #[test]
    fn message_validate_rejects_empty_parts() {
        let message = Message {
            message_id: "msg-1".to_owned(),
            context_id: None,
            task_id: None,
            role: Role::User,
            parts: Vec::new(),
            metadata: None,
            extensions: Vec::new(),
            reference_task_ids: Vec::new(),
        };

        let error = message.validate().expect_err("message should be invalid");
        assert!(
            error
                .to_string()
                .contains("message must contain at least one part")
        );
    }

    #[test]
    fn artifact_validate_rejects_empty_parts() {
        let artifact = Artifact {
            artifact_id: "artifact-1".to_owned(),
            name: None,
            description: None,
            parts: Vec::new(),
            metadata: None,
            extensions: Vec::new(),
        };

        let error = artifact.validate().expect_err("artifact should be invalid");
        assert!(
            error
                .to_string()
                .contains("artifact must contain at least one part")
        );
    }
}
