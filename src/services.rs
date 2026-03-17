use std::sync::Arc;
use crate::error::AppError;
use crate::models::{DeleteRequest, Item, UpdateStockRequest};
use crate::repositories::item_repository::ItemRepository;

pub struct ItemService{
    repository:Arc<ItemRepository>,
}
impl ItemService {
    pub fn new(repository:Arc<ItemRepository>) -> Self{
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
