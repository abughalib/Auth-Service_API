mod vars;
mod models;
mod schema;
mod errors;
mod register_handler;
mod email_service;
mod utils;
mod password_handler;
mod templates;

#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{App, HttpServer, middleware, web};
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager}
};


#[actix_web::main]
async fn main()->std::io::Result<()>{
  std::env::set_var("RUST_LOG", "actix_web=info, actix_server=info");
  env_logger::init();

  let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
  let pool: models::Pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create a database connection pool.");

  //Start Http Server
  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .wrap(middleware::Logger::default())
      .wrap(
        CookieSession::signed(&[0; 32])
          .domain(vars::domain_url().as_str())
          .name("auth")
          .secure(false)
        )
      .wrap(
        Cors::default()
          .allowed_origin("*")
          .allowed_methods(vec!["GET", "POST", "DELETE"])
          //.allowed_header()
          .max_age(3600)
      )
      .service(Files::new("/assets", "./templates/assets"))
      .service(web::scope("/"))
      .service(
        web::resource("/register/{path_id}")
        .route(web::post().to(password_handler::create_account))
      )
      .service(web::scope("/")
        .service(web::resource("/register")
          .route(web::post().to(register_handler::send_confirmation))
        )
      )
  })
  .bind(format!("{}:{}", vars::domain(), vars::port()))?
  .run()
  .await

}