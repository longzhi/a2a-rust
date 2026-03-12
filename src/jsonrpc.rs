use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const JSONRPC_VERSION: &str = "2.0";
pub const PROTOCOL_VERSION: &str = "1.0";

pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

pub const TASK_NOT_FOUND: i32 = -32001;
pub const TASK_NOT_CANCELABLE: i32 = -32002;
pub const PUSH_NOTIFICATION_NOT_SUPPORTED: i32 = -32003;
pub const UNSUPPORTED_OPERATION: i32 = -32004;
pub const CONTENT_TYPE_NOT_SUPPORTED: i32 = -32005;
pub const INVALID_AGENT_RESPONSE: i32 = -32006;
pub const EXTENDED_AGENT_CARD_NOT_CONFIGURED: i32 = -32007;
pub const EXTENSION_SUPPORT_REQUIRED: i32 = -32008;
pub const VERSION_NOT_SUPPORTED: i32 = -32009;

pub const METHOD_SEND_MESSAGE: &str = "SendMessage";
pub const METHOD_SEND_STREAMING_MESSAGE: &str = "SendStreamingMessage";
pub const METHOD_GET_TASK: &str = "GetTask";
pub const METHOD_LIST_TASKS: &str = "ListTasks";
pub const METHOD_CANCEL_TASK: &str = "CancelTask";
pub const METHOD_SUBSCRIBE_TO_TASK: &str = "SubscribeToTask";
pub const METHOD_CREATE_TASK_PUSH_NOTIFICATION_CONFIG: &str = "CreateTaskPushNotificationConfig";
pub const METHOD_GET_TASK_PUSH_NOTIFICATION_CONFIG: &str = "GetTaskPushNotificationConfig";
pub const METHOD_LIST_TASK_PUSH_NOTIFICATION_CONFIG: &str = "ListTaskPushNotificationConfig";
pub const METHOD_DELETE_TASK_PUSH_NOTIFICATION_CONFIG: &str = "DeleteTaskPushNotificationConfig";
pub const METHOD_GET_EXTENDED_AGENT_CARD: &str = "GetExtendedAgentCard";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    #[serde(default = "jsonrpc_version")]
    pub jsonrpc: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    pub id: JsonRpcId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    #[serde(default = "jsonrpc_version")]
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: JsonRpcId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcId {
    String(String),
    Number(i64),
    Null,
}

fn jsonrpc_version() -> String {
    JSONRPC_VERSION.to_owned()
}

#[cfg(test)]
mod tests {
    use super::JsonRpcId;

    #[test]
    fn jsonrpc_id_null_serializes_as_null() {
        let json = serde_json::to_string(&JsonRpcId::Null).expect("id should serialize");
        assert_eq!(json, "null");

        let round_trip: JsonRpcId = serde_json::from_str("null").expect("id should deserialize");
        assert!(matches!(round_trip, JsonRpcId::Null));
    }
}
