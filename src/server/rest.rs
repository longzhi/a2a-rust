use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::A2AError;
use crate::types::{
    AgentCard, CancelTaskRequest, CreateTaskPushNotificationConfigRequest,
    DeleteTaskPushNotificationConfigRequest, GetExtendedAgentCardRequest,
    GetTaskPushNotificationConfigRequest, GetTaskRequest, ListTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigResponse, ListTasksRequest, ListTasksResponse,
    PushNotificationConfig, SendMessageRequest, SendMessageResponse, SubscribeToTaskRequest, Task,
    TaskPushNotificationConfig,
};

use super::handler::A2AHandler;
use super::streaming;

pub(super) async fn get_agent_card<H>(
    State(handler): State<Arc<H>>,
) -> Result<Json<AgentCard>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    handler.get_agent_card().await.map(Json).map_err(rest_error)
}

pub(super) async fn send_message<H>(
    State(handler): State<Arc<H>>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<SendMessageResponse>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    request.validate()?;

    handler
        .send_message(request)
        .await
        .and_then(|response| {
            response.validate()?;
            Ok(response)
        })
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_send_message<H>(
    State(handler): State<Arc<H>>,
    Path(tenant): Path<String>,
    Json(mut request): Json<SendMessageRequest>,
) -> Result<Json<SendMessageResponse>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    request.tenant = Some(tenant);
    send_message(State(handler), Json(request)).await
}

pub(super) async fn get_task_or_subscribe<H>(
    State(handler): State<Arc<H>>,
    Path(id): Path<String>,
    Query(query): Query<GetTaskQuery>,
) -> Response
where
    H: A2AHandler,
{
    if let Err(error) = reject_query_tenant(&query.tenant) {
        return error.into_response();
    }

    if let Some(id) = id.strip_suffix(":subscribe") {
        return match streaming::subscribe_to_task_response(
            handler,
            SubscribeToTaskRequest {
                id: id.to_owned(),
                tenant: query.tenant,
            },
        )
        .await
        {
            Ok(response) => response.into_response(),
            Err(error) => error.into_response(),
        };
    }

    get_task(State(handler), Path(id), Query(query))
        .await
        .into_response()
}

pub(super) async fn tenant_get_task_or_subscribe<H>(
    State(handler): State<Arc<H>>,
    Path((tenant, id)): Path<(String, String)>,
    Query(mut query): Query<GetTaskQuery>,
) -> Response
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    if let Some(id) = id.strip_suffix(":subscribe") {
        return match streaming::subscribe_to_task_response(
            handler,
            SubscribeToTaskRequest {
                id: id.to_owned(),
                tenant: query.tenant,
            },
        )
        .await
        {
            Ok(response) => response.into_response(),
            Err(error) => error.into_response(),
        };
    }

    match handler
        .get_task(GetTaskRequest {
            id,
            history_length: query.history_length,
            tenant: query.tenant,
        })
        .await
    {
        Ok(task) => Json(task).into_response(),
        Err(error) => rest_error(error).into_response(),
    }
}

pub(super) async fn get_task<H>(
    State(handler): State<Arc<H>>,
    Path(id): Path<String>,
    Query(query): Query<GetTaskQuery>,
) -> Result<Json<Task>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    if id.ends_with(":cancel") || id.ends_with(":subscribe") {
        return Err(rest_error(A2AError::MethodNotFound("not found".to_owned())));
    }

    handler
        .get_task(GetTaskRequest {
            id,
            history_length: query.history_length,
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn list_tasks<H>(
    State(handler): State<Arc<H>>,
    Query(request): Query<ListTasksRequest>,
) -> Result<Json<ListTasksResponse>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&request.tenant)?;
    request.validate()?;

    handler
        .list_tasks(request)
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_list_tasks<H>(
    State(handler): State<Arc<H>>,
    Path(tenant): Path<String>,
    Query(mut request): Query<ListTasksRequest>,
) -> Result<Json<ListTasksResponse>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    request.tenant = Some(tenant);

    request.validate()?;

    handler
        .list_tasks(request)
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn cancel_task<H>(
    State(handler): State<Arc<H>>,
    Path(id): Path<String>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<Task>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    let Some(id) = id.strip_suffix(":cancel") else {
        return Err(rest_error(A2AError::MethodNotFound("not found".to_owned())));
    };

    handler
        .cancel_task(CancelTaskRequest {
            id: id.to_owned(),
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_cancel_task<H>(
    State(handler): State<Arc<H>>,
    Path((tenant, id)): Path<(String, String)>,
    Query(mut query): Query<TenantQuery>,
) -> Result<Json<Task>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    let Some(id) = id.strip_suffix(":cancel") else {
        return Err(rest_error(A2AError::MethodNotFound("not found".to_owned())));
    };

    handler
        .cancel_task(CancelTaskRequest {
            id: id.to_owned(),
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn get_extended_agent_card<H>(
    State(handler): State<Arc<H>>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<AgentCard>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    handler
        .get_extended_agent_card(GetExtendedAgentCardRequest {
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_get_extended_agent_card<H>(
    State(handler): State<Arc<H>>,
    Path(tenant): Path<String>,
    Query(mut query): Query<TenantQuery>,
) -> Result<Json<AgentCard>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    handler
        .get_extended_agent_card(GetExtendedAgentCardRequest {
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn create_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path(task_id): Path<String>,
    Query(query): Query<CreateTaskPushNotificationConfigQuery>,
    Json(config): Json<PushNotificationConfig>,
) -> Result<Json<TaskPushNotificationConfig>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    handler
        .create_task_push_notification_config(CreateTaskPushNotificationConfigRequest {
            task_id,
            config_id: query.config_id,
            config,
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_create_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path((tenant, task_id)): Path<(String, String)>,
    Query(mut query): Query<CreateTaskPushNotificationConfigQuery>,
    Json(config): Json<PushNotificationConfig>,
) -> Result<Json<TaskPushNotificationConfig>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    handler
        .create_task_push_notification_config(CreateTaskPushNotificationConfigRequest {
            task_id,
            config_id: query.config_id,
            config,
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn get_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path((task_id, id)): Path<(String, String)>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<TaskPushNotificationConfig>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    handler
        .get_task_push_notification_config(GetTaskPushNotificationConfigRequest {
            id,
            task_id,
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_get_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path((tenant, task_id, id)): Path<(String, String, String)>,
    Query(mut query): Query<TenantQuery>,
) -> Result<Json<TaskPushNotificationConfig>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    handler
        .get_task_push_notification_config(GetTaskPushNotificationConfigRequest {
            id,
            task_id,
            tenant: query.tenant,
        })
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn list_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path(task_id): Path<String>,
    Query(query): Query<ListTaskPushNotificationConfigQuery>,
) -> Result<Json<ListTaskPushNotificationConfigResponse>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    let request = ListTaskPushNotificationConfigRequest {
        task_id,
        page_size: query.page_size,
        page_token: query.page_token,
        tenant: query.tenant,
    };
    request.validate()?;

    handler
        .list_task_push_notification_config(request)
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn tenant_list_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path((tenant, task_id)): Path<(String, String)>,
    Query(mut query): Query<ListTaskPushNotificationConfigQuery>,
) -> Result<Json<ListTaskPushNotificationConfigResponse>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    let request = ListTaskPushNotificationConfigRequest {
        task_id,
        page_size: query.page_size,
        page_token: query.page_token,
        tenant: query.tenant,
    };
    request.validate()?;

    handler
        .list_task_push_notification_config(request)
        .await
        .map(Json)
        .map_err(rest_error)
}

pub(super) async fn delete_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path((task_id, id)): Path<(String, String)>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    reject_query_tenant(&query.tenant)?;

    handler
        .delete_task_push_notification_config(DeleteTaskPushNotificationConfigRequest {
            id,
            task_id,
            tenant: query.tenant,
        })
        .await
        .map(|()| Json(serde_json::json!({})))
        .map_err(rest_error)
}

pub(super) async fn tenant_delete_task_push_notification_config<H>(
    State(handler): State<Arc<H>>,
    Path((tenant, task_id, id)): Path<(String, String, String)>,
    Query(mut query): Query<TenantQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>
where
    H: A2AHandler,
{
    query.tenant = Some(tenant);

    handler
        .delete_task_push_notification_config(DeleteTaskPushNotificationConfigRequest {
            id,
            task_id,
            tenant: query.tenant,
        })
        .await
        .map(|()| Json(serde_json::json!({})))
        .map_err(rest_error)
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GetTaskQuery {
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(default)]
    pub history_length: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateTaskPushNotificationConfigQuery {
    pub config_id: String,
    #[serde(default)]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ListTaskPushNotificationConfigQuery {
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(default)]
    pub page_size: Option<i32>,
    #[serde(default)]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TenantQuery {
    #[serde(default)]
    pub tenant: Option<String>,
}

pub(super) fn rest_error(error: A2AError) -> (StatusCode, Json<serde_json::Value>) {
    let status = error.status_code();
    let body = serde_json::json!({
        "error": {
            "code": error.code(),
            "message": error.to_string(),
            "data": error.to_jsonrpc_error().data,
        }
    });

    (status, Json(body))
}

impl From<A2AError> for (StatusCode, Json<serde_json::Value>) {
    fn from(value: A2AError) -> Self {
        rest_error(value)
    }
}

fn reject_query_tenant(
    tenant: &Option<String>,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if tenant.is_some() {
        return Err(rest_error(A2AError::InvalidRequest(
            "tenant must be supplied via tenant-prefixed routes".to_owned(),
        )));
    }

    Ok(())
}
