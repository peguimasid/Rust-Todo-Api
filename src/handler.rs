use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::model::Todo;
use crate::model::NewTodo;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::repository;

#[get("/all")]
pub fn all_todos(connection: DbConn) -> Result<Json<Vec<Todo>>, Status> {
  repository::show_all_todos(&connection)
  .map(|todo| Json(todo))
  .map_err(|error| error_status(error))
}

#[get("/pending")]
pub fn pending_todos(connection: DbConn) -> Result<Json<Vec<Todo>>, Status> {
  repository::show_pending_todos(&connection)
    .map(|todo| Json(todo))
    .map_err(|error| error_status(error))
}

#[post("/", format ="application/json", data = "<new_todo>")]
pub fn create_todo(new_todo: Json<NewTodo>, connection: DbConn) ->  Result<status::Created<Json<Todo>>, Status> {
  println!("here 0 {}",&new_todo.title);
  repository::create_todo(new_todo.into_inner(), &connection)
    .map(|todo| todo_created(todo))
    .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_todo(id: i32, connection: DbConn) -> Result<Json<Todo>, Status> {
  repository::get_todo(id, &connection)
  .map(|todo| Json(todo))
  .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<todo>")]
pub fn update_todo(id: i32, todo: Json<Todo>, connection: DbConn) -> Result<Json<Todo>, Status> {
  repository::update_todo(id, todo.into_inner(), &connection)
  .map(|todo| Json(todo))
  .map_err(|error| error_status(error))
}


#[patch("/<id>/done")]
pub fn done_todo(id: i32, connection: DbConn) -> Result<Json<Todo>, Status> {
  repository::done_todo(id, &connection)
  .map(|todo| Json(todo))
  .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_todo(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
  repository::delete_todo(id, &connection)
  .map(|_| status::NoContent)
  .map_err(|error| error_status(error))
}

fn todo_created(todo: Todo) -> status::Created<Json<Todo>> {
  status::Created(
    format!("{host}:{port}/people/{id}", host = host(), port = port(), id = todo.id).to_string(),
    Some(Json(todo)))
}

fn error_status(error: Error) -> Status {
  match error {
      Error::NotFound => Status::NotFound,
      _ => Status::InternalServerError
  }
}

fn host() -> String {
  env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
  env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}