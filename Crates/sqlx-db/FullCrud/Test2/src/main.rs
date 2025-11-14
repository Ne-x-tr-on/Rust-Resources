use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::{env, net::SocketAddr, sync::Arc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("Application error: {0}")]
struct AppError(String);

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Car {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CarInput {
    name: String,
}

type SharedPool = Arc<PgPool>;

impl Car {
    async fn create(pool: &PgPool, name: &str) -> Result<Self, AppError> {
        let new_id = Uuid::new_v4();
        let now = Utc::now();

        let car = sqlx::query_as!(
            Car,
            r#"
            INSERT INTO cars (id, name, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, name, created_at
            "#,
            new_id,
            name,
            now
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError(e.to_string()))?;

        Ok(car)
    }

    async fn get_all(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        let cars = sqlx::query_as!(Car, "SELECT id, name, created_at FROM cars")
            .fetch_all(pool)
            .await
            .map_err(|e| AppError(e.to_string()))?;
        Ok(cars)
    }

    async fn get(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let car = sqlx::query_as!(Car, "SELECT id, name, created_at FROM cars WHERE id=$1", id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError(e.to_string()))?;
        Ok(car)
    }

    async fn update(pool: &PgPool, id: Uuid, name: &str) -> Result<Option<Self>, AppError> {
        let car = sqlx::query_as!(
            Car,
            r#"
            UPDATE cars SET name=$2
            WHERE id=$1
            RETURNING id, name, created_at
            "#,
            id,
            name
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError(e.to_string()))?;
        Ok(car)
    }

     async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!("DELETE FROM cars WHERE id=$1", id)
            .execute(pool)
            .await
            .map_err(|e| AppError(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }
}

// ------------------- AXUM HANDLERS -------------------

async fn create_car(
    State(pool): State<SharedPool>,
    Json(payload): Json<CarInput>,
) -> Result<Json<Car>, (StatusCode, String)> {
    let car = Car::create(&pool, &payload.name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(car))
}

async fn list_cars(State(pool): State<SharedPool>) -> Result<Json<Vec<Car>>, (StatusCode, String)> {
    let cars = Car::get_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(cars))
}

async fn get_car(
    State(pool): State<SharedPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Car>, (StatusCode, String)> {
    match Car::get(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        Some(car) => Ok(Json(car)),
        None => Err((StatusCode::NOT_FOUND, "Car not found".to_string())),
    }
}

async fn update_car(
    State(pool): State<SharedPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CarInput>,
) -> Result<Json<Car>, (StatusCode, String)> {
    match Car::update(&pool, id, &payload.name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        Some(car) => Ok(Json(car)),
        None => Err((StatusCode::NOT_FOUND, "Car not found".to_string())),
    }
}

async fn delete_car(
    State(pool): State<SharedPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = Car::delete(&pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Car not found".to_string()))
    }
}

// ------------------- MAIN -------------------

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").map_err(|e| AppError(e.to_string()))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    let shared_pool = Arc::new(pool);

    let app = Router::new()
        .route("/cars", post(create_car).get(list_cars))
        .route(
            "/cars/{id}",
            get(get_car).put(update_car).delete(delete_car),
        )
        .with_state(shared_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on {}", addr);

    // ---------------------- Using axum::serve(listener) ----------------------
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| AppError(e.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    Ok(())
}
