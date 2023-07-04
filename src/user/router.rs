use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};

use super::models::ModelQuery;
use super::service;

#[get("/me")]
pub async fn me(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body("me");
}

#[post("/register")]
pub async fn register(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body("register");
}

#[post("/login")]
pub async fn login(data: web::Data<AppState>, login: web::Json<ModelQuery>) -> impl Responder {
    let res = service::query(&data.db, login.into_inner()).await;
    match res {
        Ok(res) => {
            return HttpResponse::Ok().json(res);
        }
        Err(err) => {
            return HttpResponse::Ok().body(err.to_string());
        }
    }
}

#[post("/logout")]
pub async fn logout(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body("logout");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authorization")
            .service(me)
            .service(register)
            .service(login)
            .service(logout),
    );
}
