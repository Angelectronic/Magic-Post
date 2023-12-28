use actix_web::{get, web, HttpResponse, Responder, post, delete, put};
use crate::AppState;
use crate::mvc::model::logic::{insert_employee, check_employee_by_username, delete_employee_by_id, get_employee_by_id, update_employee_by_id, get_packages_by_send_point_id, get_packages_by_receive_point_id, update_employee_password_by_id, format_nested_package};
use actix_session::Session;

use crate::mvc::view::models::{SignupData, UpdateEmployee};
use crate::mvc::view::view::{view_employees, view_packages, view_package_cur_point};
use crate::mvc::model::leader::{check_leader, get_employees_by_point_id, get_cur_point_history_by_pointid};

#[post("/leader/add_employee")]
async fn add_employee(data: web::Data<AppState>, form: web::Json<SignupData>, session: Session) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let check_employees = check_employee_by_username(&mut conn, form.username.clone());
    
    let point_id = session.get::<String>("point_id").unwrap().unwrap();

    let form_point_id = form.point_id.clone().unwrap();

    if point_id != form_point_id {
        return HttpResponse::BadRequest().body("Wrong point id");
    }

    if check_employees.len() > 0 {
        return HttpResponse::BadRequest().body("Username already exists");
    } else {
        let signup_data = SignupData {
            username: form.username.clone(),
            password: form.password.clone(),
            name: form.name.clone(),
            position: form.position.clone(),
            point_id: Option::from(point_id),
        };
        let result = insert_employee(&mut conn, signup_data);
        match result {
            true => HttpResponse::Ok().body("Add employee from leader successfully"),
            false => HttpResponse::BadRequest().body("Bad request"),
        }
    }
}

#[get("/leader/view_employees")]
async fn view_employees_handler(data: web::Data<AppState>, session: Session) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = session.get::<String>("point_id").unwrap().unwrap();
    let result = get_employees_by_point_id(&mut conn, point_id);
    match result {
        Some(employees) => {
            let employees = view_employees(employees);
            HttpResponse::Ok().json(employees)
        }
        None => HttpResponse::InternalServerError().body("Error getting employees"),
    }
}

#[delete("/leader/delete_employee/{id}")]
async fn delete_employee_handler(data: web::Data<AppState>, session: Session, id: web::Path<String>) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let employee_id = id.into_inner();

    // Check if employee exists and is not leader or CEO
    let check_employees = match get_employee_by_id(&mut conn, employee_id.clone()) {
        Some(employees) => employees,
        None => return HttpResponse::BadRequest().body("Error getting employees"),
    };

    if check_employees.len() == 0 {
        return HttpResponse::BadRequest().body("Employee does not exist");
    }

    let check_employee = view_employees(check_employees)[0].clone();
    if check_employee.position == "leader" || check_employee.position == "CEO" {
        return HttpResponse::BadRequest().body("Cannot delete leader or CEO");
    }

    let result = delete_employee_by_id(&mut conn, employee_id);
    match result {
        true => HttpResponse::Ok().body("Delete employee successfully"),
        false => HttpResponse::InternalServerError().body("Error deleting employee"),
    }
}

#[put("/leader/update_employee/{id}")]
async fn update_employee_handler(data: web::Data<AppState>, session: Session, id: web::Path<String>, form: web::Json<UpdateEmployee>) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let employee_id = id.into_inner();

    // Check if employee exists and is not leader or CEO
    let check_employees = match get_employee_by_id(&mut conn, employee_id.clone()) {
        Some(employees) => employees,
        None => return HttpResponse::BadRequest().body("Employee does not exist"),
    };

    if check_employees.len() == 0 {
        return HttpResponse::BadRequest().body("Employee does not exist");
    }

    let check_employee = view_employees(check_employees)[0].clone();
    if check_employee.position == "leader" || check_employee.position == "CEO" {
        return HttpResponse::BadRequest().body("Cannot update leader or CEO");
    }

    let result = update_employee_by_id(&mut conn, form.clone(), employee_id);
    
    match result {
        true => HttpResponse::Ok().body("Update employee successfully"),
        false => HttpResponse::InternalServerError().body("Error updating employee"),
    }
}

#[put("/leader/update_employee_pass/{id}")]
async fn update_employee_pass_handler(data: web::Data<AppState>, session: Session, id: web::Path<String>, new_pass: String) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let employee_id = id.into_inner();

    // Check if employee exists and is not leader or CEO
    let check_employees = match get_employee_by_id(&mut conn, employee_id.clone()) {
        Some(employees) => employees,
        None => return HttpResponse::BadRequest().body("Employee does not exist"),
    };

    if check_employees.len() == 0 {
        return HttpResponse::BadRequest().body("Employee does not exist");
    }

    let check_employee = view_employees(check_employees)[0].clone();
    if check_employee.position == "leader" || check_employee.position == "CEO" {
        return HttpResponse::BadRequest().body("Cannot update leader or CEO");
    }

    let result = update_employee_password_by_id(&mut conn, new_pass, employee_id);
    
    match result {
        true => HttpResponse::Ok().body("Update employee password successfully"),
        false => HttpResponse::InternalServerError().body("Error updating employee password"),
    }
}

#[get("/leader/get_package_send_receive/{status}")]
async fn get_package_send_receive(data: web::Data<AppState>, session: Session, status: web::Path<String>) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = session.get::<String>("point_id").unwrap().unwrap();
    let result = match status.into_inner().as_str() {
        "send" => get_packages_by_send_point_id(&mut conn, point_id),
        "receive" => get_packages_by_receive_point_id(&mut conn, point_id),
        _ => return HttpResponse::BadRequest().body("Bad request"),
    };
        
    match result {
        Some(packages) => {
            let nested_package = format_nested_package(&mut conn, packages);
            let package = view_packages(nested_package);
            HttpResponse::Ok().json(package)
        }
        None => HttpResponse::InternalServerError().body("Error getting packages"),
    }
}

#[get("/leader/get_package_cur_history")]
async fn get_package_cur_history(data: web::Data<AppState>, session: Session) -> impl Responder {
    if !check_leader(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = session.get::<String>("point_id").unwrap().unwrap();
    
    let result = get_cur_point_history_by_pointid(&mut conn, point_id);
        
    match result {
        Some(packages_status) => {
            let mut packages_status2 = packages_status.clone();
            
            let packages = packages_status.into_iter().map(|(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost, ..)| {
                (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost)
            }).collect();

            let mut nested_package = format_nested_package(&mut conn, packages);
            let nested_package_status = nested_package.drain(..).zip(packages_status2.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, first.12, first.13, first.14, first.15, first.16, first.17, first.18, first.19, first.20, first.21, first.22, first.23, first.24, first.25, first.26, first.27, first.28, first.29, second.0, second.1)).collect::<Vec<_>>();

            let packages = view_package_cur_point(nested_package_status);
            HttpResponse::Ok().json(packages)
        }
        None => HttpResponse::InternalServerError().body("Error getting packages"),
    }
}


pub fn init_routes_leader(cfg: &mut web::ServiceConfig) {
    cfg.service(add_employee)
        .service(view_employees_handler)
        .service(delete_employee_handler)
        .service(update_employee_handler)
        .service(get_package_send_receive)
        .service(get_package_cur_history);
}