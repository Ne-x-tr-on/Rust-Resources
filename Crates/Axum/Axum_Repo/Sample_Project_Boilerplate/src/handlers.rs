use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use crate::models::{NewUser, User, NewStudent, Student};
use crate::db;
use crate::auth;

pub mod auth {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct RegisterDto {
        pub email: String,
        pub password: String,
        pub name: Option<String>,
    }

    pub async fn register(
        Extension(pool): Extension<PgPool>,
        Json(payload): Json<RegisterDto>,
    ) -> impl IntoResponse {
        // check if user exists
        if let Ok(Some(_)) = crate::db::get_user_by_email(&pool, &payload.email).await {
            return (StatusCode::CONFLICT, "User exists").into_response();
        }

        let password_hash = crate::auth::hash_password(&payload.password).unwrap();
        let new = NewUser {
            email: payload.email,
            password: payload.password, // we don't store it directly; used only for creation
            name: payload.name,
        };

        match crate::db::create_user(&pool, &new, &password_hash).await {
            Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
            Err(e) => {
                tracing::error!("create_user error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response()
            }
        }
    }

    #[derive(Deserialize)]
    pub struct LoginDto {
        pub email: String,
        pub password: String,
    }

    #[derive(serde::Serialize)]
    struct TokenResponse {
        token: String,
    }

    pub async fn login(
        Extension(pool): Extension<PgPool>,
        Json(payload): Json<LoginDto>,
    ) -> impl IntoResponse {
        match crate::db::get_user_by_email(&pool, &payload.email).await {
            Ok(Some(user)) => {
                if crate::auth::verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
                    let token = crate::auth::create_jwt(user.id).unwrap();
                    (StatusCode::OK, Json(TokenResponse { token })).into_response()
                } else {
                    (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
                }
            }
            Ok(None) => (StatusCode::NOT_FOUND, "User not found").into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
        }
    }
}

pub mod students {
    use super::*;
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize)]
    pub struct CreateStudentDto {
        pub name: String,
        pub class: String,
        pub admission_number: String,
    }

    pub async fn create_student(
        Extension(pool): Extension<PgPool>,
        Json(payload): Json<CreateStudentDto>,
    ) -> impl IntoResponse {
        let new = NewStudent {
            name: payload.name,
            class: payload.class,
            admission_number: payload.admission_number,
        };

        match crate::db::create_student(&pool, &new).await {
            Ok(student) => (StatusCode::CREATED, Json(student)).into_response(),
            Err(e) => {
                tracing::error!("create_student error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response()
            }
        }
    }

    pub async fn get_student(
        Extension(pool): Extension<PgPool>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match Uuid::parse_str(&id) {
            Ok(uuid) => match crate::db::get_student(&pool, uuid).await {
                Ok(Some(student)) => (StatusCode::OK, Json(student)).into_response(),
                Ok(None) => (StatusCode::NOT_FOUND, "Student not found").into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
            },
            Err(_) => (StatusCode::BAD_REQUEST, "Invalid UUID").into_response(),
        }
    }

    pub async fn list_students(
        Extension(pool): Extension<PgPool>,
    ) -> impl IntoResponse {
        // implement simple listing - left as exercise; use sqlx::query_as! to fetch Vec<Student>
        (StatusCode::OK, "list students - implement me").into_response()
    }
}
