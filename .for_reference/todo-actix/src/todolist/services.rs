use actix_web::{get, post, web, HttpResponse, Responder, delete , put};
use crate::{AppState, TodoListEntry};
use super::models::{CreateEntryData, UpdateEntryData};

#[get("/todolist/entries")]
async fn get_entries(app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(app_data.todolist_entry.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    let mut todolist_entries = data.todolist_entry.lock().unwrap();
    let mut max_id = 0;
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id;
        }
    }

    todolist_entries.push(TodoListEntry {
        id: max_id + 1,
        date: param_obj.date,
        title: param_obj.title.clone(),
    });

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("/todolist/entries/{id}")]
async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entry.lock().unwrap();
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = param_obj.title.clone();
            break;
        }
    }

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entry.lock().unwrap();
    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries.remove(i);
            break;
        }
    }

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}