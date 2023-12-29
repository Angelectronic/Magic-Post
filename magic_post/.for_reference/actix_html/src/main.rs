use actix_web::{web, App, HttpServer, HttpResponse, Responder, get};
use tera::{Tera, Context};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[get("/template")]
async fn template(state: web::Data<AppState>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "world");

    let rendered = state.template.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}

struct AppState {
    template: Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
        .app_data(web::Data::new(AppState { template: tera }))
        .service(index)
        .service(template)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}