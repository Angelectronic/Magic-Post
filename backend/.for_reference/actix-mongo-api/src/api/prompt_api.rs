use crate::{models::prompt_model::Prompt, repository::mongodb_repo::MongoRepo};
use actix_web::{get, web::{Data, Json, Path}, App, HttpResponse, HttpServer, Responder};

#[get("/prompt/{id}")]
pub async fn get_prompt(db: Data<MongoRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json("Invalid id");
    }
    let prompt_details = db.get_prompt(id).await;
    match prompt_details {
        Ok(prompt) => HttpResponse::Ok().json(prompt),
        Err(_) => HttpResponse::NotFound().json("Prompt not found"),
    }
}