//! SeaORM Entity. Generated by sea-orm-codegen 0.2.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_has_tags::Entity")]
    PostHasTags,
}

impl Related<super::post_has_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostHasTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
