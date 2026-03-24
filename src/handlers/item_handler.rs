use crate::error::AppError;
use crate::models::DeleteRequest;
use crate::models::Item;
use crate::models::UpdateStockRequest;
use crate::state::AppStates;
use axum::{
    Json, Router,
    extract::Query,
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchParams {
    pub name: String,
}

pub fn auth_routes() -> Router<Arc<AppStates>> {
    Router::new()
        .route("/items", get(get_items))
        .route("/items", post(add_items))
        .route("/items", patch(update_stock))
        .route("/items", delete(delete_item))
        .route("/items/search", get(find_by_name))
}

pub async fn get_items(State(state): State<Arc<AppStates>>) -> Result<Json<Vec<Item>>, AppError> {
    let items = state.item_service.get_items().await?;

    Ok(Json(items))
}

pub async fn add_items(
    State(state): State<Arc<AppStates>>,
    Json(new_item): Json<Item>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let created_item = state.item_service.add_items(new_item).await?;

    Ok((StatusCode::CREATED, Json(created_item)))
}

pub async fn update_stock(
    State(state): State<Arc<AppStates>>,
    Json(up_req): Json<UpdateStockRequest>,
) -> Result<StatusCode, AppError> {
    state.item_service.update_stock(&up_req).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_item(
    State(state): State<Arc<AppStates>>,
    Json(del_req): Json<DeleteRequest>,
) -> Result<StatusCode, AppError> {
    state.item_service.delete_item(&del_req).await?;

    Ok(StatusCode::OK)
}

pub async fn find_by_name(
    State(state): State<Arc<AppStates>>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Item>>, AppError> {
    let items = state.item_service.find_by_name(&params.name).await?;

    Ok(Json(items))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::Category,
        repositories::{item_repository::MockItemRepository, user_repository::MockUserRepository},
        services::user_service::UserService,
        services::item_service::ItemService,
        state::AppStates,

    };
    use axum::{
        body,
        extract::{Query, State},
        response::IntoResponse,
    };

    #[tokio::test]
    async fn test_get_items_handler_ok() {
        let mock_item = Item {
            id: Some(1),
            name: "モックハンドラ".to_string(),
            price: 200,
            stock: 100,
            category: Category::DailyNecessity,
        };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![mock_item],
            error_type: None,
            affected_row: 1,
        });
        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));
        let state =Arc::new(AppStates { item_service, user_service });            

        let response = get_items(State(state)).await.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let body_item: Vec<Item> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body_item.len(), 1);
        assert_eq!(body_item[0].name, "モックハンドラ");
    }

    #[tokio::test]
    async fn test_get_items_handler_internal_server_error() {
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: Some(AppError::InternalServerError("DB接続エラー".to_string())),
            affected_row: 1,
        });
        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = get_items(State(state)).await.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_add_item_ok() {
        let mock_item = Item {
            id: None,
            name: "モックレジスタ".to_string(),
            price: 100,
            stock: 10,
            category: Category::DailyNecessity,
        };

        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = add_items(State(state), Json(mock_item))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_add_item_internal_server_error() {
        let mock_item = Item {
            id: None,
            name: "モックレジスタ".to_string(),
            price: 100,
            stock: 10,
            category: Category::DailyNecessity,
        };

        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: Some(AppError::InternalServerError("DB接続エラー".to_string())),
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = add_items(State(state), Json(mock_item))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_add_item_bad_request() {
        let mock_item = Item {
            id: None,
            name: "モックレジスタ".to_string(),
            price: 0,
            stock: 10,
            category: Category::DailyNecessity,
        };

        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = add_items(State(state), Json(mock_item))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_update_stock_minus() {
        let mock_up = UpdateStockRequest { id: 1, stock: -1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = update_stock(State(state), Json(mock_up))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_update_stock_notfound() {
        let mock_up = UpdateStockRequest { id: 1, stock: 1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 0,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = update_stock(State(state), Json(mock_up))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_stock_internal_server_error() {
        let mock_up = UpdateStockRequest { id: 1, stock: 1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: Some(AppError::InternalServerError("DB接続エラー".to_string())),
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});
        let response = update_stock(State(state), Json(mock_up))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_stock_internal_ok() {
        let mock_up = UpdateStockRequest { id: 1, stock: 1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = update_stock(State(state), Json(mock_up))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_item_notfound() {
        let mock_del = DeleteRequest { id: 1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 0,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = delete_item(State(state), Json(mock_del))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_item_internal_server_error() {
        let mock_del = DeleteRequest { id: 1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: Some(AppError::InternalServerError("DB接続エラー".to_string())),
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});
        let response = delete_item(State(state), Json(mock_del))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_delete_item_ok() {
        let mock_del = DeleteRequest { id: 1 };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let response = delete_item(State(state), Json(mock_del))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_find_by_name_ok() {
        let mock_item = Item {
            id: Some(1),
            name: "モックハンドラ".to_string(),
            price: 200,
            stock: 100,
            category: Category::DailyNecessity,
        };
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![mock_item],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let mock_name = SearchParams {
            name: "モック".to_string(),
        };

        let response = find_by_name(State(state), Query(mock_name))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let body_item: Vec<Item> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body_item.len(), 1);
        assert_eq!(body_item[0].name, "モックハンドラ");
    }

    #[tokio::test]
    async fn test_find_by_name_badrequest() {
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});

        let mock_name = SearchParams {
            name: "  ".to_string(),
        };

        let response = find_by_name(State(state), Query(mock_name))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_find_by_name_notfound() {
        let mock_repo = Arc::new(MockItemRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });

        let item_service = Arc::new(ItemService::new(mock_repo));
        let user_service = Arc::new(UserService::new(Arc::new(MockUserRepository::default())));

        let state = Arc::new(AppStates{item_service,user_service});
        let mock_name = SearchParams {
            name: "該当なし".to_string(),
        };

        let response = find_by_name(State(state), Query(mock_name))
            .await
            .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
