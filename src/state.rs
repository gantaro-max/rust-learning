use crate::services::item_service::ItemService;
use crate::services::user_service::UserService;
use std::sync::Arc;
pub struct AppStates {
    pub item_service: Arc<ItemService>,
    pub user_service: Arc<UserService>,
}
