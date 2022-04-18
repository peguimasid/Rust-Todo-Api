#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/test")]
fn index() -> &'static str {
  "Hello Guilhermo"
}

#[catch(404)]
fn not_found_route() -> content::Json<&'static str> {
  content::Json("{'error': 'route not found'}")
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index])
    .register("/", catchers![not_found_route])
}
