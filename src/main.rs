#[macro_use] extern crate rocket;
use rocket::response::content;

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{\"status\": 200, \"message\":\"hello from API server\"}")
}

#[post("/alert")]
fn api_alert_receive_post() -> content::Json<&'static str> {
    content::Json("{\"status\": 200, \"message\":\"api_alert_receive_post invoked\"}")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![api_alert_receive_post])
}