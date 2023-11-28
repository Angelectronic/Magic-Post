use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;
use super::models::get_all_employees;
use super::view::view_all_employees;


#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let users: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = get_all_employees(&mut conn);
    
    let users: Vec<(String, String, String, String, String, String)> = view_all_employees(users);
    HttpResponse::Ok().body(format!("Users: {:?}", users))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}