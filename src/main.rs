#[macro_use]
extern crate rocket;

mod api;

#[launch]
fn rocket() -> _ {
	dotenvy::dotenv().expect(".env file should be present");

	let cors = rocket_cors::CorsOptions {
		..Default::default()
	}
	.to_cors()
	.expect("CORS options should be valid");

	rocket::build()
		.mount("/", routes![api::oauth::github::client_id])
		.attach(cors)
}
