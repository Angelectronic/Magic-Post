use actix_web::{get, web, HttpResponse, Responder, post, put};
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::mvc::model::ceo::{check_ceo, get_all_packages};
use crate::mvc::model::leader::check_leader;
use crate::mvc::model::subordinate::check_subordinate;
use actix_session::Session;
use crate::mvc::model::logic::{
    get_all_employees,
    check_employee_by_username,
    insert_employee,
    verify_employee_by_username_password,
    get_employee_by_id, update_employee_password_by_id, format_nested_package, get_all_deliveries, get_packages_by_delivery_id
};
use crate::mvc::view::models::{
    CreateEmployeeData,
    SignupData,
    LoginData, PackageData,
};
use crate::mvc::view::view::{view_employees, view_packages, view_delivery};

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

#[post("/login")]
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

#[put("/change_password")]
async fn update_password(data: web::Data<AppState>, new_pass: String, session: Session) -> impl Responder {
    if (!check_ceo(&session)) && (!check_leader(&session)) && (!check_subordinate(&session)) {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let id = session.get::<String>("id").unwrap().unwrap();
    
    let result = update_employee_password_by_id(&mut conn, new_pass, id);
    
    match result {
        true => HttpResponse::Ok().body("Update leader password successfully"),
        false => HttpResponse::BadRequest().body("Can't update leader password"),
    }
}

#[get("/packages/all")]
async fn get_packages(data: web::Data<AppState>, session: Session) -> impl Responder {
    if (!check_ceo(&session)) && (!check_leader(&session)) && (!check_subordinate(&session)) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let package = get_all_packages(&mut conn);        
    
    match package {
        Some(package) => {
            let nested_package = format_nested_package(&mut conn, package);
            let package: Vec<PackageData> = view_packages(nested_package);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }
}

#[get("/all_deliveries")]
async fn get_deliveries(data: web::Data<AppState>, session: Session) -> impl Responder {
    if (!check_ceo(&session)) && (!check_leader(&session)) && (!check_subordinate(&session)) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let delivery = get_all_deliveries(&mut conn);       
    
    match delivery {
        Some(delivery_raw) => {

            let delivery_package_raw = delivery_raw.into_iter().map(|(id, delivery_id, begin_date, expected_date, arrived_date, current_from, from_point_id, current_dest, dest_point_id)| {
                let convert_utf8 = |data: Option<Vec<u8>>| -> String {
                    data.map(|v| String::from_utf8(v).unwrap_or_default()).unwrap_or_default()
                };
                let delivery_id_string = convert_utf8(id.clone());

                let package = get_packages_by_delivery_id(&mut conn, delivery_id_string);
                let package = match package {
                    Some(package) => {
                        let nested_package = format_nested_package(&mut conn, package);
                        let package: Vec<PackageData> = view_packages(nested_package);
                        package
                    },
                    None => vec![]
                };
                (id, delivery_id, begin_date, expected_date, arrived_date, current_from, from_point_id, current_dest, dest_point_id, package)
            }).collect::<Vec<_>>();

            let delivery = view_delivery(delivery_package_raw);
            HttpResponse::Ok().json(delivery)
        },

        None => HttpResponse::BadRequest().body("Error getting deliveries"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_employees)
        .service(signup)
        .service(login)
        .service(update_password)
        .service(get_packages)
        .service(get_deliveries)
        .configure(init_routes_ceo)
        .configure(init_routes_leader)
        .configure(init_routes_subordinate);
}