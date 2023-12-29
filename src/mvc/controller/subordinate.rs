use actix_web::{get, web, HttpResponse, Responder, post, delete, put};
use crate::AppState;
use actix_session::Session;
use crate::mvc::view::models::{UpdatePackage, PackageData};
use crate::mvc::model::subordinate::{check_subordinate, insert_package, change_status_packaging, change_status_shipped, update_package, get_point_by_id, update_send_to_gathering, confirm_delivery};
use crate::mvc::model::logic::get_packages_by_id;


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
        send_name: form.send_name.clone(),
        send_date: form.send_date.clone(),
        send_phone: form.send_phone.clone(),
        send_address: form.send_address.clone(),
        send_point: form.send_point.clone(),
        receive_name: form.receive_name.clone(),
        receive_phone: form.receive_phone.clone(),
        receive_address: form.receive_address.clone(),
        receive_point: form.receive_point.clone(),
        from_point_id: form.from_point_id.clone(),
        dest_point_id: form.dest_point_id.clone(),
        status: form.status.clone(),
        main_cost: form.main_cost.clone(),
        other_cost: form.other_cost.clone(),
        gtgt_cost: form.gtgt_cost.clone(),
        other_service_cost: form.other_service_cost.clone(),
        total_cost: form.total_cost.clone(),
        vat: form.vat.clone(),
        package_type: form.package_type.clone(),
        instruction_type: form.instruction_type.clone(),
        weight: form.weight.clone(),
        special_service: form.special_service.clone(),
        note: form.note.clone(),
        cod: form.cod.clone(),
        receive_other_cost: form.receive_other_cost.clone(),
        items: form.items.clone()
    };

    let result = insert_package(&mut conn, new_package);
    match result {
        true => HttpResponse::Ok().body("Add package from subordinate successfully"),
        false => HttpResponse::InternalServerError().body("Cannot add package"),
    }
}

#[put("/subordinate/change_status_shipped/{package_id}")]
async fn change_status_shipped_transaction(data: web::Data<AppState>, session: Session, package_id: web::Path<String>) -> impl Responder {
    if !check_subordinate(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let package_id = package_id.into_inner();
    let point_id = session.get::<String>("point_id").unwrap().unwrap();


    let result = change_status_shipped(&mut conn, package_id, point_id);
    match result {
        true => HttpResponse::Ok().body("Change status shipped successfully"),
        false => HttpResponse::InternalServerError().body("Cannot change status shipped"),
    }
}

#[put("/subordinate/send_to_gathering/{package_id}")]
async fn send_to_gathering(data: web::Data<AppState>, session: Session, package_id: web::Path<String>) -> impl Responder {
    if !check_subordinate(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let point_id = session.get::<String>("point_id").unwrap().unwrap();
    let gathering_point_id = get_point_by_id(&mut conn, point_id.clone()).unwrap();

    let package_id = package_id.into_inner();

    let result = update_send_to_gathering(&mut conn, package_id, gathering_point_id);
    match result {
        true => HttpResponse::Ok().body("Send to gathering successfully"),
        false => HttpResponse::InternalServerError().body("Cannot send to gathering"),
    }
}

#[put("/subordinate/update/{package_id}")]
async fn update_package_handle(form: web::Json<UpdatePackage>, data: web::Data<AppState>, session: Session, package_id: web::Path<String>) -> impl Responder {
    if !check_subordinate(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let package = UpdatePackage {
        send_name: form.send_name.clone(),
        send_date: form.send_date.clone(),
        send_phone: form.send_phone.clone(),
        send_address: form.send_address.clone(),
        send_point: form.send_point.clone(),
        receive_name: form.receive_name.clone(),
        receive_phone: form.receive_phone.clone(),
        receive_address: form.receive_address.clone(),
        receive_point: form.receive_point.clone(),
        from_point_id: form.from_point_id.clone(),
        dest_point_id: form.dest_point_id.clone(),
        status: form.status.clone(),
        main_cost: form.main_cost.clone(),
        other_cost: form.other_cost.clone(),
        gtgt_cost: form.gtgt_cost.clone(),
        other_service_cost: form.other_service_cost.clone(),
        total_cost: form.total_cost.clone(),
        vat: form.vat.clone(),
        package_type: form.package_type.clone(),
        instruction_type: form.instruction_type.clone(),
        weight: form.weight.clone(),
        special_service: form.special_service.clone(),
        note: form.note.clone(),
        cod: form.cod.clone(),
        receive_other_cost: form.receive_other_cost.clone(),
        items: form.items.clone()
    };

    let package_id = package_id.into_inner();

    let result = update_package(&mut conn, package, package_id);
    match result {
        true => HttpResponse::Ok().body("Update package successfully"),
        false => HttpResponse::InternalServerError().body("Cannot change status packaging"),
    }
}

#[put("/subordinate/confirm/{delivery_id}")]
async fn confirm_package_handle(data: web::Data<AppState>, session: Session, delivery_id: web::Path<String>) -> impl Responder {
    if !check_subordinate(&session) {
        return HttpResponse::Forbidden().body("Forbidden");
    }

    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let delivery_id = delivery_id.into_inner();
    match confirm_delivery(&mut conn, delivery_id) {
        true => HttpResponse::Ok().body("Confirm delivery successfully"),
        false => HttpResponse::InternalServerError().body("Cannot confirm delivery"),
    }
}

pub fn init_routes_subordinate(cfg: &mut web::ServiceConfig) {
    cfg.service(add_package_transaction)
        .service(change_status_shipped_transaction)
        .service(update_package_handle)
        .service(send_to_gathering)
        .service(confirm_package_handle);
}