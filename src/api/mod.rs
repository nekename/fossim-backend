pub mod channels;
pub mod events;
pub mod oauth;
pub mod webhook;

use crate::Community;

use std::sync::Arc;

use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use dashmap::DashMap;
use serde::Serialize;
use tokio::sync::broadcast;

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

#[derive(Clone)]
pub struct EventChannels {
	channels: Arc<DashMap<Community, broadcast::Sender<String>>>,
}

impl EventChannels {
	pub fn new() -> Self {
		Self {
			channels: Arc::new(DashMap::new()),
		}
	}

	pub fn get_or_create_channel(&self, community: Community) -> broadcast::Sender<String> {
		self.channels
			.entry(community)
			.or_insert_with(|| broadcast::channel(32).0)
			.clone()
	}

	pub fn broadcast_to(&self, community: &Community, msg: impl Into<String>) {
		if let Some(sender) = self.channels.get(community) {
			let _ = sender.send(msg.into());
		}
	}
}
