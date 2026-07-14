use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

// ---- Structure de reponse valide ----

/// Structure représentant la réponse valide de l'application
pub struct AppResponse<T> {
    pub content: T,
    pub status_code: StatusCode,
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            axum::Json(serde_json::json!({"data": self.content})),
        )
            .into_response()
    }
}

// ---- Macro de formatage des reponses json ----
#[macro_export]
macro_rules! response {
    ($value:expr) => {
        $crate::http::AppResponse {
            content: $value,
            status_code: axum::http::StatusCode::OK,
        }
    };
    ($value:expr,$status_code:expr) => {
        $crate::http::AppResponse {
            content: $value,
            status_code: $status_code,
        }
    };
}

// ---- Structure de réponse non valide (Erreur) ----

/// Structure représentant la réponse invalide de l'application
pub enum AppError {
    Validation(String),
    Internal(anyhow::Error),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erreur interne: {err}"),
            ),
        };

        (status, axum::Json(serde_json::json!({"message": message}))).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}
