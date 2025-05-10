use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

use crate::util::{config::CONFIG, error::AppError, redis::redis_connect};

pub async fn captcha_email(user_email: &str) -> Result<(), AppError> {
    let email_send = CONFIG.email.username.clone();
    let password = CONFIG.email.password.clone();
    let host = CONFIG.email.host.clone();
    let port = CONFIG.email.port;

    let captcha_email_key = format!("captcha_email_key:{}", user_email);
    let verify_code = nanoid::nanoid!(6);

    let mut con = redis_connect();
    let _: () = redis::cmd("SET")
        .arg(captcha_email_key)
        .arg(verify_code.clone())
        .arg("EX")
        .arg(5 * 60)
        .query(&mut con)?;

    let message = Message::builder()
        .from(email_send.parse().unwrap())
        .to(user_email.parse().unwrap())
        .subject("验证码")
        .header(ContentType::TEXT_PLAIN)
        .body("你的验证码是 ".to_string() + verify_code.as_str())
        .unwrap();
    let creds = Credentials::new(email_send.to_owned(), password.to_owned());

    let mailer = match CONFIG.email.secure.as_str() {
        "tls" => SmtpTransport::relay(&host)
            .unwrap()
            .port(port)
            .credentials(creds)
            .build(),
        "starttls" => SmtpTransport::starttls_relay(&host)
            .unwrap()
            .port(port)
            .credentials(creds)
            .build(),
        _ => {
            return Err(AppError::Other);
        }
    };

    match mailer.send(&message) {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::EmailSendFail),
    }
}
