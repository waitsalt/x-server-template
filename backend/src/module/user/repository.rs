use once_cell::sync::Lazy;
use sqlx::{Pool, Postgres};

use crate::{
    module::model::SqlResult,
    util::{database::database_connect, error::AppError},
};

use super::model::User;

static POOL: Lazy<&Pool<Postgres>> = Lazy::new(|| database_connect());

pub async fn select_user_where_user_id(user_id: i64) -> SqlResult<User> {
    let sql = "
        select *
        from \"user\"
        where user_id = $1
        ";
    let result: Option<User> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(*POOL)
        .await?;
    match result {
        Some(user) => Ok(user),
        None => Err(AppError::UserNotExist),
    }
}

pub async fn select_user_where_user_name(user_name: &str) -> SqlResult<User> {
    let sql = "
        select *
        from \"user\"
        where user_name = $1
        ";
    let result: Option<User> = sqlx::query_as(sql)
        .bind(user_name)
        .fetch_optional(*POOL)
        .await?;
    match result {
        Some(user) => Ok(user),
        None => Err(AppError::UserNameNotExist),
    }
}

pub async fn select_user_where_user_email(user_email: &str) -> SqlResult<User> {
    let sql = "
        select *
        from \"user\"
        where user_email = $1
        ";
    let result: Option<User> = sqlx::query_as(sql)
        .bind(user_email)
        .fetch_optional(*POOL)
        .await?;
    match result {
        Some(user) => Ok(user),
        None => Err(AppError::UserEmailNotExist),
    }
}

pub async fn select_user_where_user_name_like(user_name: &str) -> SqlResult<Vec<User>> {
    let sql = "
        select *
        from \"user\"
        where user_name like '%$1%'
        ";
    let user_list: Vec<User> = sqlx::query_as(sql).bind(user_name).fetch_all(*POOL).await?;
    Ok(user_list)
}

pub async fn insert_user(
    name: &str,
    password: &str,
    email: &str,
    avatar_url: &str,
) -> SqlResult<()> {
    let sql = "
        insert into \"user\" (user_name, user_password, user_email, user_avatar_url)
        values ($1,$2,$3,$4)
        ";
    let _ = sqlx::query(sql)
        .bind(name)
        .bind(password)
        .bind(email)
        .bind(avatar_url)
        .execute(*POOL)
        .await?;
    Ok(())
}

pub async fn update_user_set_user_status_where_user_id(
    user_id: i64,
    user_status: i16,
) -> SqlResult<()> {
    let sql = "
        upadte \"user\"
        set user_status = $1
        where user_id = $2
        ";
    let _ = sqlx::query(sql)
        .bind(user_status)
        .bind(user_id)
        .execute(*POOL)
        .await?;
    Ok(())
}

pub async fn update_user_set_user_avator_url_where_user_id(
    id: i64,
    avatar_url: &str,
) -> SqlResult<()> {
    let sql = "
        update \"user\"
        set user_avatar_url = $1
        where user_id = $2
        ";
    let _ = sqlx::query(sql)
        .bind(avatar_url)
        .bind(id)
        .execute(*POOL)
        .await?;
    Ok(())
}

pub async fn update_user_set_user_email_where_user_id(id: i64, email: &str) -> SqlResult<()> {
    let sql = "
        update \"user\"
        set user_email = $1
        where user_id = $2
        ";
    let _ = sqlx::query(sql).bind(email).bind(id).execute(*POOL).await?;
    Ok(())
}

pub async fn update_user_set_user_password_where_user_id(id: i64, password: &str) -> SqlResult<()> {
    let sql = "
        update \"user\"
        set user_password = $1
        where user_id = $2
        ";
    let _ = sqlx::query(sql)
        .bind(password)
        .bind(id)
        .execute(*POOL)
        .await?;
    Ok(())
}
