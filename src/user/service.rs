use sea_orm::entity::prelude::*;

use super::models::{Entity as User, Model as UserModel, ModelQuery};
use sea_orm::{DatabaseConnection, DbErr};

pub async fn list(db: &DatabaseConnection) -> Result<Vec<UserModel>, DbErr> {
    User::find().all(db).await
}

pub async fn query(db: &DatabaseConnection, model: ModelQuery) -> Result<Vec<UserModel>, DbErr> {
    let mut rootFinder = User::find();

    if model.id.is_some() {
        rootFinder = rootFinder.filter(<User as sea_orm::EntityTrait>::Column::Id.eq(model.id));
    }
    if model.email.is_some() {
        rootFinder = rootFinder.filter(<User as sea_orm::EntityTrait>::Column::Email.contains(model.email.unwrap().as_str()));
    }
    if model.created_at.is_some() {
        rootFinder = rootFinder.filter(<User as sea_orm::EntityTrait>::Column::CreatedAt.eq(model.created_at));
    }
    if model.updated_at.is_some() {
        rootFinder = rootFinder.filter(<User as sea_orm::EntityTrait>::Column::UpdatedAt.eq(model.updated_at));
    }

    return rootFinder.all(db).await;
}