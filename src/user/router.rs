use actix_web::{get, post, Responder, HttpResponse, web};

#[get("/me")]
pub async fn me() -> impl Responder {
    return HttpResponse::Ok().body("me");
}

#[post("/register")]
pub async fn register() -> impl Responder {
    return HttpResponse::Ok().body("register");
}

#[post("/login")]
pub async fn login() -> impl Responder {
    return HttpResponse::Ok().body("login");
}

#[post("/logout")]
pub async fn logout() -> impl Responder {
    return HttpResponse::Ok().body("logout");
}


pub fn config(cfg:&mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
    cfg.service(me);
}