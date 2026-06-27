pub mod oauth;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, status};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ApiResponse<T> {
	Success(T),
	Error { message: String },
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T> {
	fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
		let status = match &self {
			ApiResponse::Success(_) => Status::Ok,
			ApiResponse::Error { .. } => Status::InternalServerError,
		};
		status::Custom(status, Json(self)).respond_to(req)
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
