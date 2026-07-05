pub mod oauth;
pub mod webhook;

use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ApiResponse<T> {
	Success(T),
	Error { message: String },
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
	fn into_response(self) -> Response {
		let status = match &self {
			ApiResponse::Success(_) => StatusCode::OK,
			ApiResponse::Error { .. } => StatusCode::INTERNAL_SERVER_ERROR,
		};
		(status, Json(self)).into_response()
	}
}

impl<T, E: std::fmt::Display> From<Result<T, E>> for ApiResponse<T> {
	fn from(result: Result<T, E>) -> Self {
		match result {
			Ok(val) => ApiResponse::Success(val),
			Err(e) => ApiResponse::Error {
				message: e.to_string(),
			},
		}
	}
}
