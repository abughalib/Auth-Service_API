use super::{errors::AuthError, models::Confirmation, vars};
use lettre::{transport::smtp::authentication::Credentials, Transport};
use lettre::{Message, SmtpTransport};

pub fn send_confirmation_mail(confirmation: &Confirmation, subject: &str) -> Result<(), AuthError> {
    let domain_url = vars::domain_url();
    let smtp_host = vars::smtp_host();
    let users = vars::smtp_username();
    let password = vars::smtp_password();

    let expires = confirmation
        .expires_at
        .format("%I:%M %p %A, %-d %B, %C%y")
        .to_string();

    let plain_text = format!(
        "Please visit the link below to complete registration:\n
    {domain}/register.html?id={id}&email={email}\n
    This link expires on {expires}.",
        domain = domain_url,
        id = confirmation.id,
        email = confirmation.email,
        expires = expires
    );

    let email = Message::builder()
        .to(format!("User <{}>", confirmation.email).parse().unwrap())
        .from(
            format!(
                "{} <{}>",
                vars::smtp_sender_name(),
                vars::smtp_sender_email()
            )
            .parse()
            .unwrap(),
        )
        .subject(subject)
        .body(plain_text.to_owned())
        .unwrap();

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(Credentials::new(users, password))
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent!");
            Ok(())
        }
        Err(e) => Err(AuthError::ProcessError(format!(
            "Could not send confirmation email: {e}",
        ))),
    }
}
