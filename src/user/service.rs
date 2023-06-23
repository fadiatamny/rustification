use models::Entity as User;
use models::Model as UserModel;

// add a class that on construction it receives an instance of DatabaseConnection
pub struct UserService {
    db: sea_orm::DatabaseConnection,
}

impl UserService {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self) -> Vec<UserModel> {
        let users = User::find().all(&self.db).await?;
        return users;
    }

    pub async fn find_like(&self, model: &User) -> Vec<UserModel> {
        let rootFinder = User::find();

        if model.id {
            rootFinder = rootFinder.filter(User::Column::Id.eq(model.id));
        }
        if model.name {
            rootFinder = rootFinder.filter(User::Column::Name.contains(model.name.as_str()));
        }

        return rootFinder
            .all()
            .await
            .unwrap()
    }
}

