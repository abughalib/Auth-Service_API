mod email_service;
mod errors;
mod models;
mod password_handler;
mod register_handler;
mod schema;
mod templates;
mod utils;
mod vars;
mod auth_handler;
mod tests;

#[macro_use]
extern crate diesel;
extern crate lettre;
extern crate native_tls;
extern crate serde_json;

use actix_cors::Cors;
use actix_files::Files;

use actix_session::CookieSession;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, http::header, middleware, web};
use diesel::{
  prelude::*,
  r2d2::{self, ConnectionManager},
};

async fn index(req: HttpRequest)->HttpResponse{
	if let Some(_) = req.headers().get(header::CONTENT_TYPE){
		HttpResponse::Ok().into()
	}else{
		HttpResponse::BadRequest().into()
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	
	std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
	env_logger::init();
	
	// create a database connection pool
	let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
	let pool: models::Pool = r2d2::Pool::builder()
	.build(manager)
	.expect("Failed to create a database connection pool.");
	
	// Start http server
	HttpServer::new(move || {
		App::new()
		.data(pool.clone())
		// enable logger
		.wrap(middleware::Logger::default())
		// Enable sessions
    .wrap(
      CookieSession::signed(&[0; 32])
        .secure(true),
    )
		.wrap(
			Cors::default()
      .allow_any_origin()
			.allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
			.max_age(3600)
		)
		.service(Files::new("/assets", "./templates/assets"))
		// Routes
		.service(
			web::scope("/")
    .service(web::resource("").route(web::get().to(index)))
			.service(
				web::resource("/register")
				.route(web::get().to(register_handler::show_confirmation_form))
				.route(web::post().to(register_handler::send_confirmation)),
			)
			.service(
				web::resource("/register/{path_id}")
				.route(web::get().to(password_handler::show_password_form))
				.route(web::post().to(password_handler::create_account)),
			)
			.route(
				"/signup/{path_id}",
				web::post().to(password_handler::create_account_for_browser),
			)
			.route(
				"/signup",
				web::post().to(register_handler::send_confirmation_for_browser),
			)
			.route("/me", web::get().to(auth_handler::me))
			.service(
				web::resource("/signout")
				.route(web::get().to(auth_handler::sign_out))
				.route(web::delete().to(auth_handler::sign_out)),
			)
			.service(
				web::resource("/signin")
				.route(web::get().to(auth_handler::show_sign_in_form))
				.route(web::post().to(auth_handler::sign_in)),
			)
			.route(
				"/login",
				web::post().to(auth_handler::sign_in_for_browser),
			),
		)
	})
	.bind(format!("{}:{}", vars::domain(), vars::port()))?
	.run()
	.await
}
