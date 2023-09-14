use crate::AppState;

use axum::{
    extract::State,
    response::IntoResponse
};

use shared::models::{CreateUser,User};

async fn get_user(State(state): State<AppState>, user_id: &uuid::Uuid) -> Result<User, impl IntoResponse> {
    sqlx::query_as::<_,User>(
        r#"
        SELECT id, name, email,password,created_at,ROLE
        FROM users
        WHERE user_id = $1
        "#,)
        .bind(&user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())           
}

async fn create_user(State(state): State<AppState>, create_user: &CreateUser) -> Result<User, impl IntoResponse> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email,password)
        VALUES ($1, $2, $3)
        RETURNING id, name, email,password,created_at,ROLE
        "#,
        )
        .bind(&create_user.name)
        .bind(&create_user.email)
        .bind(&create_user.password)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

async fn update_user(State(state): State<AppState>, user: &User) -> Result<User, impl IntoResponse> {
    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name, email,password, ROLE 
        WHERE id = $1
        RETURNING id, name, email,password,created_at,ROLE
        "#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

async fn delete_user(State(state): State<AppState>, user_id: &uuid::Uuid) -> Result<uuid::Uuid, impl IntoResponse> {
    sqlx::query_scalar::<_, uuid::Uuid>(
        r#"
        DELETE FROM users
        WHERE id = $1
        RETURNING id
        "#,
        )
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| e.to_string())
}
