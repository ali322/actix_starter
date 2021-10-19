use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{repository::entity::users, util::serde_format::{naive_datetime}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    #[serde(serialize_with="naive_datetime::serialize")]
    pub last_logined_at: Option<NaiveDateTime>,
    #[serde(serialize_with="naive_datetime::serialize")]
    pub created_at: Option<NaiveDateTime>,
    pub avtar: Option<String>,
    pub memo: Option<String>,
    #[serde(serialize_with="naive_datetime::serialize")]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(serialize_with="naive_datetime::serialize")]
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<users::Model> for User{
  fn from(m: users::Model) -> Self {
      Self{
        id: m.id,
        username: m.username,
        email: m.email,
        avtar: m.avtar,
        memo: m.memo,
        last_logined_at: m.last_logined_at,
        created_at: m.created_at,
        updated_at: m.updated_at,
        deleted_at: m.deleted_at
      }
  }
}
