use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;
use r2d2_mysql::mysql::prelude::*;

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let users: Vec<(String, String, String)> = conn
        .query_map(
            "SELECT * FROM customers",
            |(id, username, password)| (id, username, password),
        )
        .unwrap();
    HttpResponse::Ok().body(format!("Users: {:?}", users))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}