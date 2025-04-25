use super::error::AppError;

pub async fn captcha_phone(phone: &str) -> Result<String, AppError> {
    Ok(phone.to_string())
}
