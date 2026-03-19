use crate::error::AppError;
use crate::models::{DeleteRequest, Item, UpdateStockRequest};
use crate::repositories::item_repository::ItemRepositoryTrait;
use std::sync::Arc;
pub struct ItemService {
    repository: Arc<dyn ItemRepositoryTrait>,
}
impl ItemService {
    pub fn new(repository: Arc<dyn ItemRepositoryTrait>) -> Self {
        Self { repository }
    }
    pub async fn get_items(&self) -> Result<Vec<Item>, AppError> {
        let items = self.repository.fetch_all().await?;

        Ok(items)
    }
    pub async fn add_items(&self, new_item: Item) -> Result<Item, AppError> {
        if new_item.name.trim().is_empty() {
            return Err(AppError::BadRequest("商品名を入力してください".into()));
        }
        if new_item.price <= 0 {
            return Err(AppError::BadRequest("価格は1円以上にしてください".into()));
        }

        let created_item = self.repository.create(new_item).await?;

        Ok(created_item)
    }

    pub async fn update_stock(&self, up_req: &UpdateStockRequest) -> Result<u64, AppError> {
        if up_req.stock < 0 {
            return Err(AppError::BadRequest("在庫数は0以上にしてください".into()));
        }
        let rows = self.repository.update_stock(&up_req).await?;

        if rows == 0 {
            return Err(AppError::NotFound);
        }

        Ok(rows)
    }

    pub async fn delete_item(&self, del_req: &DeleteRequest) -> Result<u64, AppError> {
        let rows = self.repository.delete(&del_req).await?;

        if rows == 0 {
            return Err(AppError::NotFound);
        }

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Category, DeleteRequest, Item, UpdateStockRequest};
    use crate::repositories::item_repository::MockRepository;

    #[tokio::test]
    async fn test_add_item_ok() {
        let mock_item = Item {
            id: None,
            name: "モック".to_string(),
            price: 100,
            stock: 10,
            category: Category::Fruit,
        };

        let mock_repo = Arc::new(MockRepository {
            items: vec![mock_item.clone()],
            error_type: None,
            affected_row: 1,
        });
        let service = ItemService::new(mock_repo);

        let result = service.add_items(mock_item).await.unwrap();

        assert_eq!(result.name, "モック");
        assert_eq!(result.price, 100);
        assert_eq!(result.stock, 10);
        assert_eq!(result.category, Category::Fruit);
    }

    #[tokio::test]
    async fn test_add_item_empty_name() {
        let invalid_item = Item {
            id: None,
            name: "".to_string(),
            price: 100,
            stock: 10,
            category: Category::Fruit,
        };

        let mock_repo = Arc::new(MockRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });
        let service = ItemService::new(mock_repo);

        let result = service.add_items(invalid_item).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => (),
            _ => panic!("400エラーを期待していましたが違いました"),
        }
    }

    #[tokio::test]
    async fn test_get_items_db_error() {
        let mock_repo = Arc::new(MockRepository {
            items: vec![],
            error_type: Some(AppError::InternalServerError("DB接続エラー".into())),
            affected_row: 1,
        });
        let service = ItemService::new(mock_repo);

        let result = service.get_items().await;

        assert!(result.is_err());
        match result {
            Err(AppError::InternalServerError(_)) => (),
            _ => panic!("500エラーを期待していましたが違いました"),
        }
    }

    #[tokio::test]
    async fn test_update_stock_ok() {
        let mock_up = UpdateStockRequest { id: 1, stock: 10 };

        let mock_repo = Arc::new(MockRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });
        let service = ItemService::new(mock_repo);

        let result = service.update_stock(&mock_up).await.unwrap();

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_update_stock_minus() {
        let mock_up = UpdateStockRequest { id: 1, stock: -1 };
        let mock_repo = Arc::new(MockRepository {
            items: vec![],
            error_type: None,
            affected_row: 1,
        });
        let service = ItemService::new(mock_repo);

        let result = service.update_stock(&mock_up).await;

        assert!(result.is_err());
        match result {
            Err(AppError::BadRequest(_)) => (),
            _ => panic!("400エラーを期待していましたが違いました"),
        }
    }

    #[tokio::test]
    async fn test_update_stock_notfound() {
        let mock_up = UpdateStockRequest { id: 1, stock: 1 };
        let mock_repo = Arc::new(MockRepository {
            items: vec![],
            error_type: None,
            affected_row: 0,
        });
        let service = ItemService::new(mock_repo);

        let result = service.update_stock(&mock_up).await;
        assert!(result.is_err());
        match result {
            Err(AppError::NotFound) => (),
            _ => panic!("404エラーを期待していましたが違いました"),
        }
    }
    #[tokio::test]
    async fn test_delete_notfound() {
        let mock_del = DeleteRequest { id: 1 };
        let mock_repo = Arc::new(MockRepository {
            items: vec![],
            error_type: None,
            affected_row: 0,
        });
        let service = ItemService::new(mock_repo);

        let result = service.delete_item(&mock_del).await;

        assert!(result.is_err());
        match result {
            Err(AppError::NotFound) => (),
            _ => panic!("404エラーを期待していましたが違いました"),
        }
    }
}
