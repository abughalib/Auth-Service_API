use actix_session::Session;
use actix_web::{http::header::LOCATION, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use yarte::TemplateTrait;

use super::{
    errors::AuthError,
    models::{Confirmation, Pool, SessionUser, User},
    schema::{
        confirmations::dsl::{confirmations, id},
        users::dsl::users,
    },
    templates::Password,
    utils::{hash_password, is_signed_in, set_current_user, to_home},
};

#[derive(Debug, Deserialize)]
pub struct PasswordData {
    pub password: String,
}

pub async fn create_account(
    session: Session,
    path_id: web::Path<String>,
    data: web::Json<PasswordData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    if is_signed_in(&session) {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let result =
        web::block(move || create_user(&path_id.into_inner(), &data.into_inner().password, &pool))
            .await;

    match result {
        Ok(user) => {
            match user {
                Ok(usr)=>{
                    set_current_user(&session, &usr);
                    Ok(HttpResponse::Created().json(&usr))
                },
                Err(e)=>{
                    Err(e)
                }
            }
        },
        // Err(err) => match err {
        //     BlockingError::Error(auth_error) => Err(auth_error),
        //     BlockingError::Canceled => Err(AuthError::GenericError(String::from(
        //         "Could not complete the process",
        //     ))),
        // },
        Err(_e)=> Err(AuthError::GenericError(String::from(
                    "Could not complete the process",
                )))
    }
}

fn create_user(
    path_id: &str,
    password: &str,
    pool: &web::Data<Pool>,
) -> Result<SessionUser, AuthError> {
    let path_uuid = uuid::Uuid::parse_str(path_id)?;
    let conn = &pool.get().unwrap();

    confirmations
        .filter(id.eq(path_uuid))
        .load::<Confirmation>(conn)
        .map_err(|_db_err| AuthError::NotFound(String::from("Confirmation not found")))
        .and_then(|mut result| {
            if let Some(confirmation) = result.pop() {
                if confirmation.expires_at > chrono::Local::now().naive_utc() {
                    let password: String = hash_password(password)?;
                    let user: User = diesel::insert_into(users)
                        .values(&User::from(confirmation.email, password))
                        .get_result(conn)?;

                    return Ok(user.into());
                }
            }

            Err(AuthError::AuthenticationError(String::from(
                "Invalid confirmation",
            )))
        })
}

pub async fn show_password_form(
    session: Session,
    path_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    if is_signed_in(&session) {
        Ok(to_home())
    } else {
        let id_str = path_id.into_inner();

        match get_invitation(&id_str, &pool) {
            Ok(Confirmation { email, .. }) => {
                let t = Password {
                    path_id: id_str,
                    email,
                    error: None,
                };
                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(t.call().unwrap()))
            }
            Err(_) => Ok(HttpResponse::MovedPermanently()
                .append_header((LOCATION, "/register"))
                .finish()),
        }
    }
}

pub async fn create_account_for_browser(
    path_id: web::Path<String>,
    data: web::Form<PasswordData>,
    session: Session,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    let id_str = path_id.into_inner();
    let id_str2 = String::from(id_str.as_str());
    let result = web::block(move || create_user(&id_str, &data.into_inner().password, &pool)).await;

    match result {
        Ok(user) => {
            match user {
                Ok(usr)=>{
                    set_current_user(&session, &usr);
                    Ok(to_home())
                },
                Err(e)=>{
                    Err(e)
                }
            }
        }
        Err(_) => {
            let t = Password {
                path_id: id_str2,
                email: String::from("unknown@email.com"),
                error: Some(String::from("Invalid/expired confirmation id")),
            };
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(t.call().unwrap()))
        }
    }
}

fn get_invitation(path_id: &str, pool: &web::Data<Pool>) -> Result<Confirmation, AuthError> {
    let path_uuid = Uuid::parse_str(path_id)?;

    if let Ok(record) = confirmations
        .find(path_uuid)
        .get_result::<Confirmation>(&pool.get().unwrap())
    {
        Ok(record)
    } else {
        Err(AuthError::AuthenticationError(String::from(
            "Invalid confirmation",
        )))
    }
}
