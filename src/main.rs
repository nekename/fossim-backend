#[macro_use]
extern crate rocket;

mod api;

#[launch]
fn rocket() -> _ {
	dotenvy::dotenv().expect(".env file should be present");
	rocket::build().mount("/", routes![api::oauth::github::client_id])
}
