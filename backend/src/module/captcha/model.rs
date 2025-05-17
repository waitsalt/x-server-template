use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaImageResponse {
    pub captcha_image_key: String,
    pub captcha_image: String,
}

impl CaptchaImageResponse {
    pub fn new(captcha_image_key: String, captcha_image: String) -> Self {
        CaptchaImageResponse {
            captcha_image_key,
            captcha_image,
        }
    }
}
