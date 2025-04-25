use crate::util::error::AppError;

pub mod user;

type SqlResult<T> = Result<T, AppError>;
