use actix_web::{get, web, HttpResponse, Responder, post, delete, put};
use crate::AppState;
use actix_session::Session;
use crate::mvc::model::ceo::{
    check_ceo,
    get_all_points,
    get_transactions_points,
    get_gathering_points,
    get_all_leaders,
    get_leader_by_point_id,
    insert_point,
    delete_point_by_id,
    update_point,
    get_packages_at_point_id, get_packages_next_point_id
};
use crate::mvc::model::logic::{insert_employee, check_employee_by_username, delete_employee_by_id, update_employee_by_id, get_packages_by_send_point_id, get_packages_by_receive_point_id, format_nested_package};
use crate::mvc::view::models::{
    CreateEmployeeData,
    PointData,
    AddPoint, SignupData, UpdateEmployee
};
use crate::mvc::view::view::{
    view_employees,
    view_points,
    view_packages, view_packages_arrive_time,
};


#[get("/points/{point_type}")]
async fn points(data: web::Data<AppState>, point_type: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_type = point_type.into_inner();
    let points = match point_type.as_str() {
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

#[post("/add/point")]
async fn add_point(data: web::Data<AppState>, form: web::Json<AddPoint>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_data = AddPoint {
        id: Some("".to_string()),
        name: form.name.clone(),
        location: form.location.clone(),
        city: form.city.clone(),
        zipcode: form.zipcode.clone(),
        phone: form.phone.clone(),
        manager_id: form.manager_id.clone(),
        p_type: form.p_type.clone(),
        manager_reference: form.manager_reference.clone(),
        link_point_id: form.link_point_id.clone(),
        link_point_reference: form.link_point_reference.clone()
    };

    let result = insert_point(&mut conn, point_data);

    match result.as_str() {
        "Error adding point" => HttpResponse::BadRequest().body("Can't add point"),
        "Error updating manager" => HttpResponse::Ok().body("Can't update manager but added point"),
        &_ => HttpResponse::Ok().body(result)
    }
}
  
#[delete("/delete/point/{point_id}")]
async fn delete_point(data: web::Data<AppState>, point_id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = point_id.into_inner();

    let result = delete_point_by_id(&mut conn, point_id);

    match result {
        true => HttpResponse::Ok().body("Delete point successfully"),
        false => HttpResponse::BadRequest().body("Can't delete point"),
    }
}

#[put("/update/point/{point_id}")]
async fn update_points(data: web::Data<AppState>, point_id: web::Path<String>, form: web::Json<AddPoint>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = point_id.into_inner();

    let point_data = AddPoint {
        id: Some("".to_string()),
        name: form.name.clone(),
        location: form.location.clone(),
        city: form.city.clone(),
        zipcode: form.zipcode.clone(),
        phone: form.phone.clone(),
        manager_id: form.manager_id.clone(),
        p_type: '1'.to_string(),
        manager_reference: form.manager_reference.clone(),
        link_point_id: form.link_point_id.clone(),
        link_point_reference: form.link_point_reference.clone()
    };

    let result = update_point(&mut conn, point_data, point_id);

    match result {
        true => HttpResponse::Ok().body("Update point successfully"),
        false => HttpResponse::BadRequest().body("Can't update point"),
    }
}

#[get("/leaders/{point_id}")]
async fn leaders(data: web::Data<AppState>, point_id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = point_id.into_inner();
    let leaders = match point_id.as_str() {
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

#[post("/add/leader")]
async fn add_leader(data: web::Data<AppState>, form: web::Json<SignupData>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    match &form.position {
        Some(position) => {
            if position != "leader" {
                return HttpResponse::BadRequest().body("Must be leader");
            }
        },
        None => return HttpResponse::BadRequest().body("No position provided"),
    }

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
            position: Some(String::from("leader")),
            point_id: form.point_id.clone(),
        };
        let result = insert_employee(&mut conn, signup_data);
        match result {
            true => HttpResponse::Ok().body("Add leader successfully"),
            false => HttpResponse::BadRequest().body("Bad request"),
        }
    }
}

#[delete("/delete/leader/{id}")]
async fn delete_leader(data: web::Data<AppState>, id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let id = id.into_inner();

    let result = delete_employee_by_id(&mut conn, id);

    match result {
        true => HttpResponse::Ok().body("Delete leader successfully"),
        false => HttpResponse::BadRequest().body("Can't delete leader"),
    }
}

#[put("/update/leader/{id}")]
async fn update_leaders(data: web::Data<AppState>, id: web::Path<String>, form: web::Json<UpdateEmployee>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let id = id.into_inner();

    let result = update_employee_by_id(&mut conn, form.clone(), id);

    match result {
        true => HttpResponse::Ok().body("Update leader successfully"),
        false => HttpResponse::BadRequest().body("Can't update leader"),
    }
}

#[get("/packages/send/{point_id}")]
async fn get_packages_send(data: web::Data<AppState>, point_id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let point_id = point_id.into_inner();
    let package = get_packages_by_send_point_id(&mut conn, point_id);
    
    match package {
        Some(package) => {
            let nested_package = format_nested_package(&mut conn, package);
            let package = view_packages(nested_package);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }   
}

#[get("/packages/receive/{point_id}")]
async fn get_packages_receive(data: web::Data<AppState>, point_id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let point_id = point_id.into_inner();
    let package = get_packages_by_receive_point_id(&mut conn, point_id);
    
    match package {
        Some(package) => {
            let nested_package = format_nested_package(&mut conn, package);
            let package = view_packages(nested_package);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }   
}

#[get("/packages/at/{point_id}")]
async fn get_packages_at(data: web::Data<AppState>, point_id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let point_id = point_id.into_inner();
    let package = get_packages_at_point_id(&mut conn, point_id);
    
    match package {
        Some(packages_status) => {
            let mut packages_status2 = packages_status.clone();
            
            let packages = packages_status.into_iter().map(|(id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost, ..)| {
                (id, package_id, send_name, send_date, send_phone, send_address, send_point, receive_name, receive_phone, receive_address, receive_point, current_from, from_point_id, current_dest, dest_point_id, package_status, main_cost, other_cost, gtgt_cost, other_service_cost, total_cost, vat, package_type, instruction_type, weight, special_service, note, cod, receive_other_cost)
            }).collect();

            let mut nested_package = format_nested_package(&mut conn, packages);
            let nested_package_status = nested_package.drain(..).zip(packages_status2.drain(..)).map(|(first, second)| (first.0, first.1, first.2, first.3, first.4, first.5, first.6, first.7, first.8, first.9, first.10, first.11, first.12, first.13, first.14, first.15, first.16, first.17, first.18, first.19, first.20, first.21, first.22, first.23, first.24, first.25, first.26, first.27, first.28, second.29, first.29)).collect::<Vec<_>>();

            let package = view_packages_arrive_time(nested_package_status);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }   
}

#[get("/packages/to/{point_id}")]
async fn get_packages_to(data: web::Data<AppState>, point_id: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }
    
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let point_id = point_id.into_inner();
    let package = get_packages_next_point_id(&mut conn, point_id);
    
    match package {
        Some(package) => {
            let nested_package = format_nested_package(&mut conn, package);
            let package = view_packages(nested_package);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }   
}


pub fn init_routes_ceo(config: &mut web::ServiceConfig) {
    config.service(points);
    config.service(add_point);
    config.service(delete_point);
    config.service(update_points);
    config.service(leaders);
    config.service(add_leader);
    config.service(delete_leader);
    config.service(update_leaders);
    config.service(get_packages_send);
    config.service(get_packages_receive);
    config.service(get_packages_at);
    config.service(get_packages_to);
}