use hmac::{Hmac, KeyInit, Mac};
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload, WebhookEventType};
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct GitHubEvent(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GitHubEvent {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match req.headers().get_one("X-GitHub-Event") {
			Some(event) => Outcome::Success(GitHubEvent(event.to_string())),
			None => Outcome::Error((Status::BadRequest, ())),
		}
	}
}

pub struct GitHubSignature(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GitHubSignature {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		match req.headers().get_one("X-Hub-Signature-256") {
			Some(sig) => Outcome::Success(GitHubSignature(sig.to_string())),
			None => Outcome::Error((Status::Forbidden, ())),
		}
	}
}

/// Verifies `sha256=<hex>` against HMAC-SHA256(secret, body).
/// Returns Err(()) on any malformed input or mismatch.
fn verify_signature(secret: &str, signature_header: &str, body: &[u8]) -> Result<(), ()> {
	let hex_sig = signature_header.strip_prefix("sha256=").ok_or(())?;
	let sig_bytes = hex::decode(hex_sig).map_err(|_| ())?;

	let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| ())?;
	mac.update(body);
	mac.verify_slice(&sig_bytes).map_err(|_| ())
}

#[post("/api/webhook/github", data = "<body>")]
pub async fn webhook(
	event_type: GitHubEvent,
	signature: GitHubSignature,
	body: Data<'_>,
) -> Status {
	let bytes = match body.open(2_i32.mebibytes()).into_bytes().await {
		Ok(b) if b.is_complete() => b.into_inner(),
		_ => return Status::PayloadTooLarge,
	};

	let secret = match std::env::var("WEBHOOK_GITHUB_SECRET") {
		Ok(s) => s,
		Err(_) => {
			warn!("WEBHOOK_GITHUB_SECRET not set");
			return Status::InternalServerError;
		}
	};

	if verify_signature(&secret, &signature.0, &bytes).is_err() {
		warn!("Webhook signature verification failed");
		return Status::Forbidden;
	}

	let event = match WebhookEvent::try_from_header_and_body(&event_type.0, &bytes) {
		Ok(e) => e,
		Err(err) => {
			warn!("Failed to parse webhook payload: {err}");
			return Status::BadRequest;
		}
	};

	match event.kind {
		WebhookEventType::Discussion => {
			let WebhookEventPayload::Discussion(payload) = event.specific else {
				return Status::BadRequest;
			};

			info!("Received discussion event: {:?}", payload.action);
		}
		WebhookEventType::DiscussionComment => {
			let WebhookEventPayload::DiscussionComment(payload) = event.specific else {
				return Status::BadRequest;
			};

			info!("Received discussion comment event: {:?}", payload.action);
		}
		_ => {}
	}

	Status::Ok
}
