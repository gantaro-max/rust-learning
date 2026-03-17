use std::sync::Arc;
use crate::error::AppError;
use crate::models::{DeleteRequest, Item, UpdateStockRequest};
use crate::repositories::item_repository::{ItemRepository, ItemRepositoryTrait};
pub struct ItemService{
    repository:Arc<dyn ItemRepositoryTrait>,
}
impl ItemService {
    pub fn new(repository:Arc<dyn ItemRepositoryTrait>) -> Self{
        Self { repository }
        
    }
    pub async fn get_items(&self)->Result<Vec<Item>,AppError>{
        let items = self.repository.fetch_all().await?;

        Ok(items)
    }    
    pub async fn add_items(&self,new_item:Item)->Result<Item,AppError>{
        if new_item.name.trim().is_empty(){
            return Err(AppError::BadRequest("商品名を入力してください".into()));
        }
        if new_item.price<=0{
            return Err(AppError::BadRequest("価格は1円以上にしてください".into()));
        }
    
        let created_item = self.repository.create(new_item).await?;
    
        Ok(created_item)
    }

    pub async fn update_stock(&self,up_req:&UpdateStockRequest)->Result<u64,AppError>{
        if up_req.stock<0 {
            return Err(AppError::BadRequest("在庫数は0以上にしてください".into()));            
        }
        let row=self.repository.update_stock(&up_req).await?;
        
        Ok(row)
    }

    pub async fn delete_item(&self,del_req:&DeleteRequest)->Result<u64,AppError>{
        let row=self.repository.delete(&del_req).await?;

        Ok(row)
    }


}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::models::{Category, Item};
    use sqlx::PgPool;
    use async_trait::async_trait;

    struct MockRepository;

    #[async_trait]
    impl ItemRepositoryTrait for MockRepository {
        async fn create(&self, item: Item) -> Result<Item, AppError> {
            Ok(item) // 投げたものをそのまま返すだけ（DB操作なし！）
        }
        async fn fetch_all(&self) -> Result<Vec<Item>, AppError> {
            Ok(vec![]) // 空のリストを返す
        }
        async fn update_stock(&self,up_req:&UpdateStockRequest)->Result<u64,AppError>{
            Ok(1)
        }
        async fn delete(&self,del_req:&DeleteRequest)->Result<u64,AppError>{
            Ok(1)
        }
        
    }

    #[tokio::test]
    async fn test_add_item_empty_name(){
        let pool = PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let repo = Arc::new(ItemRepository::new(pool));
        let service = ItemService::new(repo);

        let invalid_item = Item {
            id:None,
            name: "".to_string(),
            price: 100,
            stock: 10,
            category: Category::Fruit,

        };

        let result = service.add_items(invalid_item).await;

        assert!(result.is_err());
        if let Err(AppError::BadRequest(msg)) = result{
            assert_eq!(msg,"商品名を入力してください");
        }
        else{
            panic!("BadRequestエラーが返ってくるはず");
        }
        
    }
}
