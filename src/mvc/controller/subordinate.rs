use actix_web::{get, web, HttpResponse, Responder, post, delete, put};
use crate::AppState;
use actix_session::Session;
use crate::mvc::view::models::{UpdatePackage};
use crate::mvc::model::subordinate::{check_subordinate, insert_package};


#[post("/subordinate/add_package")]
async fn add_package_transaction(form: web::Json<UpdatePackage>, data: web::Data<AppState>, session: Session) -> impl Responder {
    if !check_subordinate(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = session.get::<String>("point_id").unwrap().unwrap();
    let form_point_id = form.send_point.clone().unwrap();

    if point_id != form_point_id {
        return HttpResponse::BadRequest().body("Wrong point id");
    }

    let new_package = UpdatePackage {
        send_point: Option::from(point_id.clone()),
        receive_point: form.receive_point.clone(),
        cur_point: Option::from(point_id.clone()),
        status: Option::from("Pending".to_string()),
        send_name: form.send_name.clone(),
        send_date: form.send_date.clone(),
        required_date: form.required_date.clone(),
        shipped_date: form.shipped_date.clone(),
        send_address: form.send_address.clone(),
        receive_address: form.receive_address.clone(),
        send_phone: form.send_phone.clone(),
        receive_phone: form.receive_phone.clone(),
        receive_name: form.receive_name.clone()
    };

    let result = insert_package(&mut conn, new_package);
    match result {
        true => HttpResponse::Ok().body("Add package from subordinate successfully"),
        false => HttpResponse::InternalServerError().body("Cannot add package"),
    }

}



pub fn init_routes_subordinate(cfg: &mut web::ServiceConfig) {
    cfg.service(add_package_transaction);
}