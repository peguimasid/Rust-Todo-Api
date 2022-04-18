#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/hello/<name>/<age>/<cool>")]
fn index(name: &str, age: u8, cool: bool) -> String {
  if cool {
    format!("You're a cool {} year old, {}!", age, name)
  } else {
    format!("{}, we need to talk about coolness.", name)
  }
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
