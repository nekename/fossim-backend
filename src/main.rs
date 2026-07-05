mod api;

use axum::{
	Router,
	routing::{get, post},
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	dotenvy::dotenv().expect(".env file should be present");

	let app = Router::new()
		.route(
			"/api/oauth/github/client_id",
			get(api::oauth::github::client_id),
		)
		.route("/api/webhook/github", post(api::webhook::github::webhook))
		.layer(CorsLayer::permissive());

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
