use actix_session::Session;
use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use serde::Deserialize;
use yarte::TemplateTrait;

use {
    super::email_service::send_confirmation_mail,
    super::errors::AuthError,
    super::models::{Confirmation, Pool},
    super::templates::Register,
    super::utils::{is_signed_in, to_home},
};

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
}

pub async fn send_confirmation(
    //session: Session,
    data: web::Json<RegisterData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    if is_signed_in(&session) {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let result = web::block(move || create_confirmation(data.into_inner().email, &pool)).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(AuthError::GenericError(String::from(
            "Could not complete the process",
        ))),
    }
}

pub async fn show_confirmation_form(session: Session) -> Result<HttpResponse, AuthError> {
    if is_signed_in(&session) {
        Ok(to_home())
    } else {
        let template = Register {
            sent: false,
            error: None,
        };

        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(template.call().unwrap()))
    }
}

pub async fn send_confirmation_for_browser(
    data: web::Form<RegisterData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    let result = web::block(move || create_confirmation(data.into_inner().email, &pool)).await;
    let template = match result {
        Ok(_) => Register {
            sent: true,
            error: None,
        },
        Err(_) => Register {
            sent: false,
            error: Some(String::from("Could not complete the process")),
        },
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(template.call().unwrap()))
}

fn create_confirmation(email: String, pool: &web::Data<Pool>) -> Result<(), AuthError> {
    let confirmation = insert_record(email, pool)?;
    let subject: &str = "Registration Confirmation Mail";
    send_confirmation_mail(&confirmation, subject)
}

fn insert_record(email: String, pool: &web::Data<Pool>) -> Result<Confirmation, AuthError> {
    use super::schema::confirmations::dsl::confirmations;

    let new_record: Confirmation = email.into();

    let inserted_record = diesel::insert_into(confirmations)
        .values(&new_record)
        .get_result(&mut pool.get().unwrap())?;

    Ok(inserted_record)
}
