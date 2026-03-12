use http::StatusCode;
use serde_json::Value;
use thiserror::Error;

use crate::jsonrpc;
use crate::jsonrpc::JsonRpcError;

#[derive(Debug, Error)]
pub enum A2AError {
    #[error("task not found: {0}")]
    TaskNotFound(String),
    #[error("task not cancelable: {0}")]
    TaskNotCancelable(String),
    #[error("push notification not supported: {0}")]
    PushNotificationNotSupported(String),
    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("content type not supported: {0}")]
    ContentTypeNotSupported(String),
    #[error("invalid agent response: {0}")]
    InvalidAgentResponse(String),
    #[error("extended agent card not configured: {0}")]
    ExtendedAgentCardNotConfigured(String),
    #[error("extension support required: {0}")]
    ExtensionSupportRequired(String),
    #[error("version not supported: {0}")]
    VersionNotSupported(String),
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("method not found: {0}")]
    MethodNotFound(String),
    #[error("invalid params: {0}")]
    InvalidParams(String),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[cfg(feature = "client")]
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

impl A2AError {
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

    pub fn to_jsonrpc_error(&self) -> JsonRpcError {
        JsonRpcError {
            code: self.code(),
            message: self.to_string(),
            data: self.data(),
        }
    }

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
