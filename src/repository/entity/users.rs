//! SeaORM Entity. Generated by sea-orm-codegen 0.2.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub username: Option<String>,
    pub password: String,
    pub email: Option<String>,
    pub last_logined_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    #[sea_orm(column_type = "Text", nullable)]
    pub avtar: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub memo: Option<String>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::posts::Entity")]
    Posts,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
