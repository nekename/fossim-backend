pub mod github;

use super::*;

#[derive(Serialize)]
pub struct ClientIdResponse {
	client_id: String,
}
