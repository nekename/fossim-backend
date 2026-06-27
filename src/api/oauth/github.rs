use super::*;

use std::env;

#[get("/api/oauth/github/client_id")]
pub fn client_id() -> ApiResponse<ClientIdResponse> {
	env::var("OAUTH_GITHUB_CLIENT_ID")
		.map(|client_id| ClientIdResponse { client_id })
		.map_err(|e| e.to_string())
		.into()
}
