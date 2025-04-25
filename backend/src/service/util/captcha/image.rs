use captcha_rust::Captcha;
use nanoid::nanoid;
use redis::Commands;

use crate::{
    model::util::CaptchaImageResponse,
    util::{AppResult, redis::redis_connect, response::AppResponse},
};

pub async fn image() -> AppResult<CaptchaImageResponse> {
    let captcha = Captcha::new(5, 100, 40);
    let mut con = redis_connect();
    let captcha_image_key = nanoid!(16);
    let _: () = con
        .set_ex(
            format!("captcha_image_key:{}", captcha_image_key),
            captcha.text,
            5 * 60,
        )
        .unwrap();
    Ok(AppResponse::success(Some(CaptchaImageResponse::new(
        captcha_image_key,
        captcha.base_img,
    ))))
}
