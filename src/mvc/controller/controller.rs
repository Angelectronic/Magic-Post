use actix_web::{get, web, HttpResponse, Responder, post};
use serde::{Serialize, Deserialize};
use crate::AppState;
use actix_session::Session;
use crate::mvc::model::logic::{
    get_all_employees,
    check_employee_by_username,
    insert_employee,
    verify_employee_by_username_password,
    get_employee_by_id
};
use crate::mvc::view::models::{
    CreateEmployeeData,
    SignupData,
    LoginData,
};
use crate::mvc::view::view::{view_employees};

use super::ceo::init_routes_ceo;
use super::leader::init_routes_leader;
use super::subordinate::init_routes_subordinate;

#[get("/all_employees")]
async fn all_employees(data: web::Data<AppState>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let employees = match get_all_employees(&mut conn) {
        Some(employees) => employees,
        None => return HttpResponse::BadRequest().body("Error getting employees"),
    };
    
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
        let convert_utf8 = |data: Option<Vec<u8>>| -> String {
            data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
        };

        let id = convert_utf8(login_employee[0].0.clone());
        let position = convert_utf8(login_employee[0].2.clone());
        let point_id = convert_utf8(login_employee[0].3.clone());
        
        session.insert("id", id.clone()).unwrap();
        session.insert("position", position.clone()).unwrap();
        session.insert("point_id", point_id.clone()).unwrap();

        if position == "CEO" {
            #[derive(Serialize, Clone, Debug, Deserialize)]
            struct CEOposition {
                position: String,
            }
            let position = CEOposition {
                position: "CEO".to_string(),
            };

            return HttpResponse::Ok().json(position);
        } else {
            let login_send_back = get_employee_by_id(&mut conn, id.clone());
            let login_send_back = match login_send_back {
                Some(login_send_back) => login_send_back,
                None => return HttpResponse::BadRequest().body("Error getting login_send_back"),
            };

            let login_send_back = view_employees(login_send_back);
            let login_send_back = login_send_back[0].clone();

            HttpResponse::Ok().json(login_send_back)
        }
    } else {
        HttpResponse::Forbidden().body("Wrong username or password")
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_employees)
        .service(signup)
        .service(login)
        .configure(init_routes_ceo)
        .configure(init_routes_leader)
        .configure(init_routes_subordinate);
}