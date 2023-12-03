use actix_web::{get, web, HttpResponse, Responder, post};
use crate::AppState;
use actix_session::Session;
use crate::mvc::model::logic::{
    get_all_employees,
    check_employee_by_username,
    insert_employee,
    verify_employee_by_username_password,
};
use crate::mvc::view::models::{
    CreateEmployeeData,
    SignupData,
    LoginData,
};
use crate::mvc::view::view::view_employees;

use super::ceo::{points, leaders, add_point, delete_point, update_points, add_leader, delete_leader, update_leaders, get_packages, get_packages_send, get_packages_receive};
use super::leader::add_employee;

#[get("/all_employees")]
async fn all_employees(data: web::Data<AppState>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let employees: Vec<(Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>, Option<Vec<u8>>)> = get_all_employees(&mut conn);
    
    let employees: Vec<CreateEmployeeData> = view_employees(employees);
    HttpResponse::Ok().json(employees)
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
        .service(login)
        .service(add_point)
        .service(delete_point)
        .service(update_points)
        .service(add_leader)
        .service(delete_leader)
        .service(update_leaders)
        .service(get_packages)
        .service(get_packages_send)
        .service(add_employee)
        .service(get_packages_receive);       
}