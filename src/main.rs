use std::sync::Arc;

use actix_web::{HttpResponse, web, Responder, get};
use r2d2_mysql::{
    mysql::{prelude::*, Opts, OptsBuilder},
    r2d2, MySqlConnectionManager,
};

fn get_pool() -> Option<Arc<r2d2::Pool<MySqlConnectionManager>>> {
    let url = "mysql://root:@localhost:3306/magic_post";
    let opts = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());
    Some(pool)
}

struct AppState {
    pool: Arc<r2d2::Pool<MySqlConnectionManager>>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let pool = data.pool.clone();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    let users: Vec<(String, String, String)> = conn
        .query_map(
            "SELECT * FROM customers",
            |(id, username, password)| (id, username, password),
        )
        .unwrap();
    HttpResponse::Ok().body(format!("Users: {:?}", users))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().unwrap();
    let data = web::Data::new(AppState { pool });
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(data.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}