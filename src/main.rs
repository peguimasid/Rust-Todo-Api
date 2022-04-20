#[macro_use]
extern crate rocket;

use rocket::response::content;

use rocket::fs::TempFile;

#[post("/upload", format = "multipart", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
  file.persist_to("./tmp").await
}

#[get("/hello/<name>/<age>/<cool>", format = "application/json")]
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
    .mount("/", routes![index, upload])
    .register("/", catchers![not_found_route])
}
