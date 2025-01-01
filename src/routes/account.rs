use crate::db::AppState;
use crate::models::account::payload::{LoginPayload, SignupPayload};
use crate::models::account::response::{LoginResponse, SignupResponse};
use axum::{routing::post, Extension, Json, Router};
use entity::account_user::{self, Entity as AccountUserEntity};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

async fn signup(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<SignupPayload>,
) -> Json<SignupResponse> {
    let db = &state.db;
    let existing_user = AccountUserEntity::find()
        .filter(account_user::Column::Email.eq(payload.email.clone()))
        .one(db)
        .await;
    match existing_user {
        Ok(Some(_)) => {
            return Json(SignupResponse {
                success: false,
                message: "Email already exists".to_string(),
            });
        }
        Ok(None) => {
            let new_user = account_user::ActiveModel {
                uuid: Set(uuid::Uuid::new_v4()), // Assuming `uuid` is a string
                first_name: Set(payload.first_name),
                last_name: Set(payload.last_name),
                email: Set(payload.email),
                password: Set(payload.password),
                role: Set("hacker".to_string()),
                is_active: Set(false),
            };

            match new_user.insert(db).await {
                Ok(_) => Json(SignupResponse {
                    success: true,
                    message: "Signup successful".to_string(),
                }),
                Err(err) => {
                    eprintln!("Failed to insert user: {:?}", err);
                    Json(SignupResponse {
                        success: false,
                        message: "Failed to create account".to_string(),
                    })
                }
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Json(SignupResponse {
                success: false,
                message: "Internal server error".to_string(),
            })
        }
    }
}
async fn login(Json(payload): Json<LoginPayload>) -> Json<LoginResponse> {
    if payload.email.len() > 0 && payload.password.len() > 0 {
        Json(LoginResponse {
            success: true,
            message: "Success".to_string(),
        })
    } else {
        Json(LoginResponse {
            success: false,
            message: "Internal server error".to_string(),
        })
    }
}
