mod vars;
mod models;
mod schema;
mod errors;
mod register_handler;
mod email_service;
mod utils;

#[macro_use]
use diesel;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};
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
  })
  .bind(format!("{}:{}", vars::domain(), vars::port()))?
  .run()
  .await

}