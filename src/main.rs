mod api;
mod channels;

use axum::{
	Router,
	extract::FromRef,
	routing::{any, get, post},
};
use tower_http::cors::CorsLayer;

#[derive(PartialEq, Eq, Hash)]
struct Community(String, String, String);

#[derive(Clone)]
struct AppState {
	api_event_channels: api::EventChannels,
	channels: channels::Channels,
}

impl FromRef<AppState> for api::EventChannels {
	fn from_ref(state: &AppState) -> Self {
		state.api_event_channels.clone()
	}
}

impl FromRef<AppState> for channels::Channels {
	fn from_ref(state: &AppState) -> Self {
		state.channels.clone()
	}
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	let _ = dotenvy::dotenv();

	let app = Router::new()
		.route(
			"/api/oauth/github/client_id",
			get(api::oauth::github::client_id),
		)
		.route("/api/webhook/github", post(api::webhook::github::webhook))
		.route(
			"/api/events/{forge}/{author}/{repo}",
			any(api::events::events),
		)
		.route(
			"/api/channels/{forge}/{author}/{repo}/seq_counters",
			post(api::channels::seq_counters),
		)
		.layer(CorsLayer::permissive())
		.with_state(AppState {
			api_event_channels: api::EventChannels::new(),
			channels: channels::Channels::new(),
		});

	let port = std::env::var("PORT")
		.unwrap_or_else(|_| "8000".to_string())
		.parse::<u16>()
		.expect("PORT should be a valid u16");
	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
		.await
		.expect("listener should be able to bind to the port");

	axum::serve(listener, app)
		.await
		.expect("server should be able to start");
}
