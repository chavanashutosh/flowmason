use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
    Extension,
};
use uuid::Uuid;
use serde::Deserialize;
use flowmason_auth::AuthContext;

use crate::dto::{CreateTemplateRequest, TemplateResponse, UpdateTemplateRequest, InstantiateTemplateRequest, PaginationParams, PaginatedResponse};
use crate::routes::TemplateState;
use flowmason_core::types::{Flow, BrickConfig};

pub fn routes() -> Router<TemplateState> {
    Router::new()
        .route("/", post(create_template).get(list_templates))
        .route("/:id", get(get_template).put(update_template).delete(delete_template))
        .route("/:id/instantiate", post(instantiate_template))
        .route("/categories", get(list_categories))
}

#[derive(Deserialize)]
struct TemplateQueryParams {
    category: Option<String>,
    include_system: Option<bool>,
}

async fn create_template(
    State(state): State<TemplateState>,
    Extension(auth_context): Extension<AuthContext>,
    Json(payload): Json<CreateTemplateRequest>,
) -> Result<Json<TemplateResponse>, StatusCode> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let template = flowmason_core::types::Template {
        id: id.clone(),
        name: payload.name,
        description: payload.description,
        category: payload.category,
        flow_config: payload.flow_config,
        is_system: false,
        created_by: Some(auth_context.user_id),
        created_at: now,
        updated_at: now,
    };
    
    state.template_repo.create(&template).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(TemplateResponse::from(template)))
}

async fn list_templates(
    State(state): State<TemplateState>,
    Query(params): Query<PaginationParams>,
    Query(template_params): Query<TemplateQueryParams>,
) -> Result<Json<PaginatedResponse<TemplateResponse>>, StatusCode> {
    let include_system = template_params.include_system.unwrap_or(true);
    let templates = state.template_repo.list(
        template_params.category.as_deref(),
        include_system,
        Some(params.limit),
        Some(params.offset)
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let items = templates.into_iter().map(TemplateResponse::from).collect();
    Ok(Json(PaginatedResponse::new(items, params.limit, params.offset)))
}

async fn get_template(
    State(state): State<TemplateState>,
    Path(id): Path<String>,
) -> Result<Json<TemplateResponse>, StatusCode> {
    let template = state.template_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(TemplateResponse::from(template)))
}

async fn update_template(
    State(state): State<TemplateState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTemplateRequest>,
) -> Result<Json<TemplateResponse>, StatusCode> {
    let mut template = state.template_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Only allow updating user templates
    if template.is_system {
        return Err(StatusCode::FORBIDDEN);
    }

    // Only allow updating own templates
    if template.created_by.as_ref() != Some(&auth_context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }

    if let Some(name) = payload.name {
        template.name = name;
    }
    if let Some(description) = payload.description {
        template.description = Some(description);
    }
    if let Some(category) = payload.category {
        template.category = category;
    }
    if let Some(flow_config) = payload.flow_config {
        template.flow_config = flow_config;
    }
    template.updated_at = chrono::Utc::now();

    state.template_repo.update(&template).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(TemplateResponse::from(template)))
}

async fn delete_template(
    State(state): State<TemplateState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let template = state.template_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Only allow deleting user templates
    if template.is_system {
        return Err(StatusCode::FORBIDDEN);
    }

    // Only allow deleting own templates
    if template.created_by.as_ref() != Some(&auth_context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }

    state.template_repo.delete(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn instantiate_template(
    State(state): State<TemplateState>,
    Path(id): Path<String>,
    Json(payload): Json<InstantiateTemplateRequest>,
) -> Result<Json<crate::dto::FlowResponse>, StatusCode> {
    let template = state.template_repo.get(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let flow_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    
    let mut flow = template.flow_config.clone();
    flow.id = flow_id;
    flow.name = payload.name.unwrap_or_else(|| format!("{} (Copy)", template.name));
    flow.description = payload.description.or(template.description);
    flow.created_at = now;
    flow.updated_at = now;
    
    state.flow_repo.create(&flow).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(crate::dto::FlowResponse::from(flow)))
}

async fn list_categories(
    State(state): State<TemplateState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let categories = state.template_repo.list_categories().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(categories))
}
