use argonautica::{Hasher, Verifier};
use actix_session::Session;
use actix_web::{HttpRequest, http::header::CONTENT_TYPE};

use super::{errors::AuthError, vars, models::SessionUser};

pub fn hash_password(password: &str)->Result<String, AuthError>{
  Hasher::default()
    .with_password(password)
    .with_secret_key(vars::secret_key().as_str())
    .hash()
    .map_err(|_| AuthError::AuthenticationError(
      String::from("could not hash password")))
}

pub fn verify(hash: &str, password: &str) -> Result<bool, AuthError>{
  Verifier::default()
    .with_hash(hash)
    .with_password(password)
    .with_secret_key(vars::secret_key())
    .verify()
    .map_err(|_| AuthError::AuthenticationError(
      String::from("Could not verify password")))
}

pub fn is_json_request(req: &HttpRequest)->bool{
  req
    .headers()
    .get(CONTENT_TYPE)
    .map_or(
      false,
      |header| header.to_str().map_or(false, |content_type|
      "application/json" == content_type)
    )
}

pub fn set_current_user(session: &Session, user: &SessionUser){
  session.set("user",
    serde_json::to_string(user).unwrap()
  ).unwrap();
}

pub fn get_current_user(session: &Session)->Result<SessionUser, AuthError>{
  let err = AuthError::AuthenticationError(
    String::from("Could not retrieve user from session"));
  let session_result = session.get::<String>("user");

    if session_result.is_err(){
      return Err(err.clone());
    }
    session_result
      .unwrap()
      .map_or(Err(err),
        |user_str| serde_json::from_str(&user_str).or_else(|_|
            Err(err)))
}