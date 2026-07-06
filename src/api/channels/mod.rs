use super::*;

use crate::Community;
use crate::channels::Channels;

use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};

#[derive(serde::Deserialize)]
pub struct SeqCounterRequest {
	channel_ids: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct SeqCounterResponse {
	seq_counters: HashMap<String, u64>,
}

pub async fn seq_counters(
	Path((forge, author, repo)): Path<(String, String, String)>,
	State(channels): State<Channels>,
	Json(request): Json<SeqCounterRequest>,
) -> ApiResponse<SeqCounterResponse> {
	if request.channel_ids.len() > 200 {
		return ApiResponse::Error {
			message: "Too many channels requested".to_string(),
		};
	}
	let community = Community(forge, author, repo);
	channels
		.get_seq_counters(&community, request.channel_ids)
		.map(|seq_counters| SeqCounterResponse { seq_counters })
		.map_err(|e| e.to_string())
		.into()
}
