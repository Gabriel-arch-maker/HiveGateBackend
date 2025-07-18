use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation    )]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}