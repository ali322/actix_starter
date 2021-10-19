use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::repository::{DBConn, DBError, entity::users, vo};
use sea_orm::{entity::*, query::*};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct QueryUser {
    pub key: Option<String>,
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 1))]
    limit: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl QueryUser{
  pub async fn find_all(&self, conn: &DBConn) -> Result<(usize, Vec<vo::User>), DBError> {
    let page = self.page.unwrap_or(1);
    let limit = self.limit.unwrap_or(10);
    // let conn = init_db().await;
    let paginator = users::Entity::find().paginate(conn, limit as usize);
    let num_pages = paginator.num_pages().await?;
    let rows: Vec<vo::User> = paginator.fetch_page((page - 1) as usize).await?.into_iter().map(Into::into).collect();
    Ok((num_pages, rows))
  }
}