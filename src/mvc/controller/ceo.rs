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
    update_point, delete_employee_by_id,
    update_employee_by_id,
    get_all_packages,
    get_packages_by_send_point_id,
    get_packages_by_receive_point_id
};
use crate::mvc::model::logic::{insert_employee, check_employee_by_username};
use crate::mvc::view::models::{
    CreateEmployeeData,
    PointData,
    AddPoint, SignupData, UpdateEmployee, PackageData
};
use crate::mvc::view::view::{
    view_employees,
    view_points,
    view_packages,
};


#[get("/points/{point_type}")]
async fn points(data: web::Data<AppState>, point_type: web::Path<String>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

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

#[post("/add/point")]
async fn add_point(data: web::Data<AppState>, form: web::Json<AddPoint>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_data = AddPoint {
        location: form.location.clone(),
        p_type: form.p_type.clone(),
    };

    let location = match point_data.location {
        Some(location) => location,
        None => return HttpResponse::BadRequest().body("Error at location"),
    };

    let p_type = match point_data.p_type {
        Some(p_type) => p_type,
        None => return HttpResponse::BadRequest().body("Error at p_type"),
    };

    let result = insert_point(&mut conn, location, p_type);

    match result {
        true => HttpResponse::Ok().body("Add point successfully"),
        false => HttpResponse::BadRequest().body("Can't add point"),
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
        location: form.location.clone(),
        p_type: form.p_type.clone(),
    };

    let location = match point_data.location {
        Some(location) => location,
        None => return HttpResponse::BadRequest().body("Error at location"),
    };

    let p_type = match point_data.p_type {
        Some(p_type) => p_type,
        None => return HttpResponse::BadRequest().body("Error at p_type"),
    };

    let result = update_point(&mut conn, point_id, location, p_type);

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

    let result = update_employee_by_id(&mut conn, id, form.name.clone(), form.position.clone(), form.point_id.clone());

    match result {
        true => HttpResponse::Ok().body("Update leader successfully"),
        false => HttpResponse::BadRequest().body("Can't update leader"),
    }
}

#[get("/packages/all")]
async fn get_packages(data: web::Data<AppState>, session: Session) -> impl Responder {
    if !check_ceo(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    let package = get_all_packages(&mut conn);
    
    match package {
        Some(package) => {
            let package: Vec<PackageData> = view_packages(package);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
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
            let package: Vec<PackageData> = view_packages(package);
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
            let package: Vec<PackageData> = view_packages(package);
            HttpResponse::Ok().json(package)
        },

        None => HttpResponse::BadRequest().body("Bad request"),
    }   
}
