#[macro_use]
extern crate rocket;

use rocket::response::content;
use rocket::tokio::time::{sleep, Duration};

#[get("/test")]
fn index() -> &'static str {
  "Hello Guilhermo"
}

#[get("/sleep/<seconds>")]
async fn delay(seconds: u64) -> String {
  sleep(Duration::from_secs(seconds)).await;
  format!("Program sleeped {} seconds", seconds)
}

#[catch(404)]
fn not_found_route() -> content::Json<&'static str> {
  content::Json("{'error': 'route not found'}")
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index, delay])
    .register("/", catchers![not_found_route])
}
