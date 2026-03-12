use http::StatusCode;
use serde_json::Value;
use thiserror::Error;

use crate::jsonrpc;
use crate::jsonrpc::JsonRpcError;

/// Unified error type for A2A protocol, HTTP, and serialization failures.
#[derive(Debug, Error)]
pub enum A2AError {
    /// The requested task identifier does not exist.
    #[error("task not found: {0}")]
    TaskNotFound(String),
    /// The requested task cannot transition to canceled.
    #[error("task not cancelable: {0}")]
    TaskNotCancelable(String),
    /// Push notifications are disabled or unsupported for this agent.
    #[error("push notification not supported: {0}")]
    PushNotificationNotSupported(String),
    /// The requested operation is not implemented.
    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
    /// The request content type is not supported by the peer.
    #[error("content type not supported: {0}")]
    ContentTypeNotSupported(String),
    /// The remote agent returned an invalid response payload.
    #[error("invalid agent response: {0}")]
    InvalidAgentResponse(String),
    /// Extended agent card retrieval is not configured for the agent.
    #[error("extended agent card not configured: {0}")]
    ExtendedAgentCardNotConfigured(String),
    /// A required extension is not supported by the peer.
    #[error("extension support required: {0}")]
    ExtensionSupportRequired(String),
    /// The peer rejected the requested A2A protocol version.
    #[error("version not supported: {0}")]
    VersionNotSupported(String),
    /// The request body could not be parsed as valid JSON-RPC or JSON.
    #[error("parse error: {0}")]
    ParseError(String),
    /// The request shape is structurally invalid.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// The requested method or route does not exist.
    #[error("method not found: {0}")]
    MethodNotFound(String),
    /// The supplied parameters could not be deserialized or validated.
    #[error("invalid params: {0}")]
    InvalidParams(String),
    /// An internal SDK or server error occurred.
    #[error("internal error: {0}")]
    Internal(String),
    /// JSON serialization or deserialization failed locally.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[cfg(feature = "client")]
    /// The underlying HTTP client returned an error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

impl A2AError {
    /// Return the JSON-RPC error code associated with this error.
    pub fn code(&self) -> i32 {
        match self {
            Self::TaskNotFound(_) => jsonrpc::TASK_NOT_FOUND,
            Self::TaskNotCancelable(_) => jsonrpc::TASK_NOT_CANCELABLE,
            Self::PushNotificationNotSupported(_) => jsonrpc::PUSH_NOTIFICATION_NOT_SUPPORTED,
            Self::UnsupportedOperation(_) => jsonrpc::UNSUPPORTED_OPERATION,
            Self::ContentTypeNotSupported(_) => jsonrpc::CONTENT_TYPE_NOT_SUPPORTED,
            Self::InvalidAgentResponse(_) => jsonrpc::INVALID_AGENT_RESPONSE,
            Self::ExtendedAgentCardNotConfigured(_) => jsonrpc::EXTENDED_AGENT_CARD_NOT_CONFIGURED,
            Self::ExtensionSupportRequired(_) => jsonrpc::EXTENSION_SUPPORT_REQUIRED,
            Self::VersionNotSupported(_) => jsonrpc::VERSION_NOT_SUPPORTED,
            Self::ParseError(_) => jsonrpc::PARSE_ERROR,
            Self::InvalidRequest(_) => jsonrpc::INVALID_REQUEST,
            Self::MethodNotFound(_) => jsonrpc::METHOD_NOT_FOUND,
            Self::InvalidParams(_) => jsonrpc::INVALID_PARAMS,
            Self::Internal(_) => jsonrpc::INTERNAL_ERROR,
            Self::Serialization(_) => jsonrpc::INTERNAL_ERROR,
            #[cfg(feature = "client")]
            Self::Http(_) => jsonrpc::INTERNAL_ERROR,
        }
    }

    /// Convert this error into a JSON-RPC error object.
    pub fn to_jsonrpc_error(&self) -> JsonRpcError {
        JsonRpcError {
            code: self.code(),
            message: self.to_string(),
            data: self.data(),
        }
    }

    /// Return the HTTP status code associated with this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::TaskNotFound(_) => StatusCode::NOT_FOUND,
            Self::TaskNotCancelable(_) => StatusCode::CONFLICT,
            Self::PushNotificationNotSupported(_) => StatusCode::BAD_REQUEST,
            Self::UnsupportedOperation(_) => StatusCode::BAD_REQUEST,
            Self::ContentTypeNotSupported(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::InvalidAgentResponse(_) => StatusCode::BAD_GATEWAY,
            Self::ExtendedAgentCardNotConfigured(_) => StatusCode::BAD_REQUEST,
            Self::ExtensionSupportRequired(_) => StatusCode::BAD_REQUEST,
            Self::VersionNotSupported(_) => StatusCode::BAD_REQUEST,
            Self::ParseError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            Self::MethodNotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidParams(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Serialization(_) => StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(feature = "client")]
            Self::Http(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn data(&self) -> Option<Value> {
        match self {
            Self::TaskNotFound(task_id) => Some(Value::String(task_id.clone())),
            Self::TaskNotCancelable(task_id) => Some(Value::String(task_id.clone())),
            Self::PushNotificationNotSupported(detail)
            | Self::UnsupportedOperation(detail)
            | Self::ContentTypeNotSupported(detail)
            | Self::InvalidAgentResponse(detail)
            | Self::ExtendedAgentCardNotConfigured(detail)
            | Self::ExtensionSupportRequired(detail)
            | Self::VersionNotSupported(detail)
            | Self::ParseError(detail)
            | Self::InvalidRequest(detail)
            | Self::MethodNotFound(detail)
            | Self::InvalidParams(detail)
            | Self::Internal(detail) => Some(Value::String(detail.clone())),
            Self::Serialization(_) => None,
            #[cfg(feature = "client")]
            Self::Http(_) => None,
        }
    }
}
