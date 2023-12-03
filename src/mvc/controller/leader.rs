use actix_web::{get, web, HttpResponse, Responder, post, delete, put};
use crate::AppState;
use crate::mvc::model::logic::{insert_employee, check_employee_by_username};
use actix_session::Session;

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
use crate::mvc::model::leader::{check_leader};

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
            true => HttpResponse::Ok().body("Signup successfully"),
            false => HttpResponse::BadRequest().body("Bad request"),
        }
    }
}