use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// DB Table without Primary Key
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}



// Query Partial object

#[derive(Deserialize)]
pub struct ModelQuery {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}