use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;
use super::models::{get_all_employees, get_all_points, get_transactions_points, get_gathering_points, get_all_leaders, get_leader_by_point_id};
use super::view::{view_employees, view_points, CreateEmployeeData, PointData};


#[get("/employees")]
async fn employees(data: web::Data<AppState>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let employees: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = get_all_employees(&mut conn);
    
    let employees: Vec<CreateEmployeeData> = view_employees(employees);
    HttpResponse::Ok().json(employees)
}

#[get("/points/{point_type}")]
async fn points(data: web::Data<AppState>, point_type: web::Path<String>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_type = point_type.into_inner();
    let points: Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<i8>)>> = match point_type.as_str() {
        "transactions" => get_transactions_points(&mut conn),
        "gathering" => get_gathering_points(&mut conn),
        "all" => get_all_points(&mut conn),
        _ => None,
    };
    
    match points {
        Some(points) => {
            let points: Vec<PointData> = view_points(points);
            HttpResponse::Ok().json(points)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }
  }

#[get("/leaders/{point_id}")]
async fn leaders(data: web::Data<AppState>, point_id: web::Path<String>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = point_id.into_inner();
    let leaders: Option<Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)>> = match point_id.as_str() {
        "all" => get_all_leaders(&mut conn),
        _ => get_leader_by_point_id(&mut conn, point_id),
    };

    match leaders {
        Some(leaders) => {
            let leaders: Vec<CreateEmployeeData> = view_employees(leaders);
            HttpResponse::Ok().json(leaders)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(employees)
        .service(points)
        .service(leaders);
}