use actix_web::middleware::Logger;
use env_logger::Env;
use std::time;
use std::sync::Mutex;

mod user;

async fn connect_to_db() {
    use sea_orm::{Database, ConnectOptions};
    let mut opt = ConnectOptions::new("mysql://root:root@localhost:3333/gate".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(time::Duration::from_secs(8))
        .acquire_timeout(time::Duration::from_secs(8))
        .idle_timeout(time::Duration::from_secs(8))
        .max_lifetime(time::Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await;
    return db;
}

struct AppState {
    db: Mutex<sea_orm::DatabaseConnection>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let db = connect_to_db().await;

    // let app_state = AppState {
    //     db: Mutex::new(connect_to_db().await),
    // };

    // This initializes the env to run in info mode
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .configure(user::router::config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}