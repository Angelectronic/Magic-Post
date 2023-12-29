mod mvc;
use mvc::controller::controller as controller;
use std::sync::Arc;
use actix_web::{web::{self}, cookie::{Key, SameSite}};
use r2d2_mysql::{
    mysql::{Opts, OptsBuilder},
    r2d2, MySqlConnectionManager,
};
use actix_session::{ SessionMiddleware, config::{ BrowserSession, CookieContentSecurity }, storage::CookieSessionStore };
use actix_cors::Cors;

struct AppState {
    pool: Arc<r2d2::Pool<MySqlConnectionManager>>
}

// connect to mysql
fn get_pool() -> Option<Arc<r2d2::Pool<MySqlConnectionManager>>> {
    let url = "mysql://root:@localhost:3306/magic_post";
    let opts = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());
    Some(pool)
}


fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64])
    )
	.cookie_name(String::from("session")) 
	.cookie_secure(true) 
	.session_lifecycle(BrowserSession::default()) 
	.cookie_same_site(SameSite::Strict) 
	.cookie_content_security(CookieContentSecurity::Private) 
	.cookie_http_only(true)
	.build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().unwrap();
    let data = web::Data::new(AppState { pool });
    actix_web::HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        actix_web::App::new()
            .app_data(data.clone())
            .configure(controller::config)
            .wrap(session_middleware())
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}