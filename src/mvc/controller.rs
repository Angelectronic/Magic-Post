use actix_session::storage::{CookieSessionStore, SessionStore};
use actix_web::{get, web, HttpResponse, Responder, post};
use crate::AppState;
use super::models::{get_all_employees, get_all_points, get_transactions_points, get_gathering_points, get_all_leaders, get_leader_by_point_id, check_employee_by_username, insert_employee, verify_employee_by_username_password};
use super::view::{view_employees, view_points, CreateEmployeeData, PointData, SignupData, LoginData};
use actix_session::Session;


#[get("/all_employees")]
async fn all_employees(data: web::Data<AppState>, session: Session) -> impl Responder {
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


#[post("/signup")]
async fn signup(data: web::Data<AppState>, form: web::Json<SignupData>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let check_employees = check_employee_by_username(&mut conn, form.username.clone());
    
    if check_employees.len() > 0 {
        return HttpResponse::BadRequest().body("Username already exists");
    } else {
        let signup_data = SignupData {
            username: form.username.clone(),
            password: form.password.clone(),
            name: form.name.clone(),
            position: form.position.clone(),
            point_id: form.point_id.clone(),
        };
        let result = insert_employee(&mut conn, signup_data);
        match result {
            true => HttpResponse::Ok().body("Signup successfully"),
            false => HttpResponse::BadRequest().body("Bad request"),
        }
    }
}

#[get("/login")]
async fn login(data: web::Data<AppState>, form: web::Json<LoginData>, session: Session) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let login_employee = verify_employee_by_username_password(&mut conn, form.username.clone(), form.password.clone());

    if login_employee.len() > 0 {
        let login_employee = view_employees(login_employee);
        
        session.insert("id", login_employee[0].id.clone()).unwrap();
        session.insert("position", login_employee[0].position.clone()).unwrap();
        session.insert("point_id", login_employee[0].point_id.clone()).unwrap();
        
        HttpResponse::Ok().body("Login successfully")
    } else {
        HttpResponse::Forbidden().body("Wrong username or password")
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_employees)
        .service(points)
        .service(leaders)
        .service(signup)
        .service(login);
}