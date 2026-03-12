use std::pin::Pin;

use async_trait::async_trait;
use futures_core::Stream;

use crate::A2AError;
use crate::types::{
    AgentCard, CancelTaskRequest, CreateTaskPushNotificationConfigRequest,
    DeleteTaskPushNotificationConfigRequest, GetExtendedAgentCardRequest,
    GetTaskPushNotificationConfigRequest, GetTaskRequest, ListTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigResponse, ListTasksRequest, ListTasksResponse,
    SendMessageRequest, SendMessageResponse, StreamResponse, SubscribeToTaskRequest, Task,
    TaskPushNotificationConfig,
};

pub type A2AStream = Pin<Box<dyn Stream<Item = StreamResponse> + Send + 'static>>;

/// Core server trait for implementing an A2A agent.
///
/// The default capability helpers call `get_agent_card()` on each gated request.
/// Implementations that fetch the card from storage should cache it or override
/// the relevant operation methods.
#[async_trait]
pub trait A2AHandler: Send + Sync + 'static {
    async fn get_agent_card(&self) -> Result<AgentCard, A2AError>;

    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError>;

    /// Stream responses for a submitted message.
    ///
    /// Message-only flows should emit exactly one `StreamResponse::Message`.
    /// Task-based flows should emit the initial task first, followed by status
    /// and artifact updates until the stream closes.
    async fn send_streaming_message(
        &self,
        _request: SendMessageRequest,
    ) -> Result<A2AStream, A2AError> {
        self.require_streaming_capability("SendStreamingMessage")
            .await?;
        Err(A2AError::UnsupportedOperation(
            "SendStreamingMessage".to_owned(),
        ))
    }

    async fn get_task(&self, _request: GetTaskRequest) -> Result<Task, A2AError> {
        Err(A2AError::UnsupportedOperation("GetTask".to_owned()))
    }

    async fn list_tasks(&self, _request: ListTasksRequest) -> Result<ListTasksResponse, A2AError> {
        Err(A2AError::UnsupportedOperation("ListTasks".to_owned()))
    }

    async fn cancel_task(&self, _request: CancelTaskRequest) -> Result<Task, A2AError> {
        Err(A2AError::UnsupportedOperation("CancelTask".to_owned()))
    }

    /// Subscribe to updates for an existing task.
    ///
    /// Implementations must emit the current `StreamResponse::Task` first before
    /// any subsequent status or artifact updates.
    async fn subscribe_to_task(
        &self,
        _request: SubscribeToTaskRequest,
    ) -> Result<A2AStream, A2AError> {
        self.require_streaming_capability("SubscribeToTask").await?;
        Err(A2AError::UnsupportedOperation("SubscribeToTask".to_owned()))
    }

    async fn create_task_push_notification_config(
        &self,
        _request: CreateTaskPushNotificationConfigRequest,
    ) -> Result<TaskPushNotificationConfig, A2AError> {
        self.require_push_notifications_capability("CreateTaskPushNotificationConfig")
            .await?;
        Err(A2AError::UnsupportedOperation(
            "CreateTaskPushNotificationConfig".to_owned(),
        ))
    }

    async fn get_task_push_notification_config(
        &self,
        _request: GetTaskPushNotificationConfigRequest,
    ) -> Result<TaskPushNotificationConfig, A2AError> {
        self.require_push_notifications_capability("GetTaskPushNotificationConfig")
            .await?;
        Err(A2AError::UnsupportedOperation(
            "GetTaskPushNotificationConfig".to_owned(),
        ))
    }

    async fn list_task_push_notification_config(
        &self,
        _request: ListTaskPushNotificationConfigRequest,
    ) -> Result<ListTaskPushNotificationConfigResponse, A2AError> {
        self.require_push_notifications_capability("ListTaskPushNotificationConfig")
            .await?;
        Err(A2AError::UnsupportedOperation(
            "ListTaskPushNotificationConfig".to_owned(),
        ))
    }

    async fn delete_task_push_notification_config(
        &self,
        _request: DeleteTaskPushNotificationConfigRequest,
    ) -> Result<(), A2AError> {
        self.require_push_notifications_capability("DeleteTaskPushNotificationConfig")
            .await?;
        Err(A2AError::UnsupportedOperation(
            "DeleteTaskPushNotificationConfig".to_owned(),
        ))
    }

    async fn get_extended_agent_card(
        &self,
        _request: GetExtendedAgentCardRequest,
    ) -> Result<AgentCard, A2AError> {
        self.require_extended_agent_card_capability().await?;
        Err(A2AError::ExtendedAgentCardNotConfigured(
            "GetExtendedAgentCard".to_owned(),
        ))
    }

    /// Enforce the A2A streaming capability gate.
    ///
    /// Do not override unless you preserve the same protocol behavior.
    async fn require_streaming_capability(&self, operation: &str) -> Result<(), A2AError> {
        let card = self.get_agent_card().await?;
        if card.capabilities.streaming == Some(true) {
            return Ok(());
        }

        Err(A2AError::UnsupportedOperation(operation.to_owned()))
    }

    /// Enforce the A2A push-notifications capability gate.
    ///
    /// Do not override unless you preserve the same protocol behavior.
    async fn require_push_notifications_capability(&self, operation: &str) -> Result<(), A2AError> {
        let card = self.get_agent_card().await?;
        if card.capabilities.push_notifications == Some(true) {
            return Ok(());
        }

        Err(A2AError::PushNotificationNotSupported(operation.to_owned()))
    }

    /// Enforce the A2A extended-agent-card capability gate.
    ///
    /// Do not override unless you preserve the same protocol behavior.
    async fn require_extended_agent_card_capability(&self) -> Result<(), A2AError> {
        let card = self.get_agent_card().await?;
        if card.capabilities.extended_agent_card == Some(true) {
            return Ok(());
        }

        Err(A2AError::ExtendedAgentCardNotConfigured(
            "GetExtendedAgentCard".to_owned(),
        ))
    }
}
