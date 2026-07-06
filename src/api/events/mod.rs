use super::EventChannels;

use crate::Community;

use axum::extract::{
	Path, State, WebSocketUpgrade,
	ws::{Message, WebSocket},
};
use axum::response;

pub async fn events(
	Path((forge, author, repo)): Path<(String, String, String)>,
	State(api_event_channels): State<EventChannels>,
	ws: WebSocketUpgrade,
) -> impl response::IntoResponse {
	if !matches!(forge.as_str(), "github") {
		return response::Response::builder()
			.status(axum::http::StatusCode::NOT_FOUND)
			.body(axum::body::Body::empty())
			.unwrap();
	}
	ws.on_upgrade(move |socket| handle_socket(socket, forge, author, repo, api_event_channels))
}

async fn handle_socket(
	mut socket: WebSocket,
	forge: String,
	author: String,
	repo: String,
	api_event_channels: EventChannels,
) {
	let community = Community(forge, author, repo);
	let mut rx = api_event_channels
		.get_or_create_channel(community)
		.subscribe();

	while let Ok(msg) = rx.recv().await {
		if socket.send(Message::Text(msg.into())).await.is_err() {
			break;
		}
	}
}
