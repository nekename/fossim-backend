use axum::{
	body::Bytes,
	http::{HeaderMap, StatusCode},
};
use hmac::{Hmac, KeyInit, Mac};
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload, WebhookEventType};
use sha2::Sha256;
use tracing::{info, warn};

type HmacSha256 = Hmac<Sha256>;

/// Verifies `sha256=<hex>` against HMAC-SHA256(secret, body).
/// Returns Err(()) on any malformed input or mismatch.
fn verify_signature(secret: &str, signature_header: &str, body: &[u8]) -> Result<(), ()> {
	let hex_sig = signature_header.strip_prefix("sha256=").ok_or(())?;
	let sig_bytes = hex::decode(hex_sig).map_err(|_| ())?;

	let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| ())?;
	mac.update(body);
	mac.verify_slice(&sig_bytes).map_err(|_| ())
}

pub async fn webhook(headers: HeaderMap, body: Bytes) -> StatusCode {
	let Some(event_type) = headers.get("X-GitHub-Event").and_then(|v| v.to_str().ok()) else {
		return StatusCode::BAD_REQUEST;
	};

	let Some(signature) = headers
		.get("X-Hub-Signature-256")
		.and_then(|v| v.to_str().ok())
	else {
		return StatusCode::FORBIDDEN;
	};

	let secret = match std::env::var("WEBHOOK_GITHUB_SECRET") {
		Ok(s) => s,
		Err(_) => {
			warn!("WEBHOOK_GITHUB_SECRET not set");
			return StatusCode::INTERNAL_SERVER_ERROR;
		}
	};

	if verify_signature(&secret, signature, &body).is_err() {
		warn!("Webhook signature verification failed");
		return StatusCode::FORBIDDEN;
	}

	let event = match WebhookEvent::try_from_header_and_body(event_type, &body) {
		Ok(e) => e,
		Err(err) => {
			warn!("Failed to parse webhook payload: {err}");
			return StatusCode::BAD_REQUEST;
		}
	};

	match event.kind {
		WebhookEventType::Discussion => {
			let WebhookEventPayload::Discussion(payload) = event.specific else {
				return StatusCode::BAD_REQUEST;
			};

			info!("Received discussion event: {:?}", payload.action);
		}
		WebhookEventType::DiscussionComment => {
			let WebhookEventPayload::DiscussionComment(payload) = event.specific else {
				return StatusCode::BAD_REQUEST;
			};

			info!("Received discussion comment event: {:?}", payload.action);
		}
		_ => {}
	}

	StatusCode::OK
}
