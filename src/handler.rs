use crate::connection::DbConn;
use diesel::result::Error;
use std::env;
use crate::model::Task;
use crate::model::NewTask;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::repository;

#[get("/all")]
pub fn all_tasks(connection: DbConn) -> Result<Json<Vec<Task>>, Status> {
  repository::show_all_tasks(&connection)
  .map(|task| Json(task))
  .map_err(|error| error_status(error))
}

#[get("/pending")]
pub fn pending_tasks(connection: DbConn) -> Result<Json<Vec<Task>>, Status> {
  repository::show_pending_tasks(&connection)
    .map(|task| Json(task))
    .map_err(|error| error_status(error))
}

#[post("/new", format ="application/json", data = "<new_task>")]
pub fn create_task(new_task: Json<NewTask>, connection: DbConn) ->  Result<status::Created<Json<Task>>, Status> {
  println!("here 0 {}",&new_task.title);
  repository::create_task(new_task.into_inner(), &connection)
    .map(|task| task_created(task))
    .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_task(id: i32, connection: DbConn) -> Result<Json<Task>, Status> {
  repository::get_task(id, &connection)
  .map(|task| Json(task))
  .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<task>")]
pub fn update_task(id: i32, task: Json<Task>, connection: DbConn) -> Result<Json<Task>, Status> {
  repository::update_task(id, task.into_inner(), &connection)
  .map(|task| Json(task))
  .map_err(|error| error_status(error))
}


#[patch("/<id>/done")]
pub fn done_task(id: i32, connection: DbConn) -> Result<Json<Task>, Status> {
  repository::done_task(id, &connection)
  .map(|task| Json(task))
  .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_task(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
  repository::delete_task(id, &connection)
  .map(|_| status::NoContent)
  .map_err(|error| error_status(error))
}

fn task_created(task: Task) -> status::Created<Json<Task>> {
  status::Created(
    format!("{host}:{port}/people/{id}", host = host(), port = port(), id = task.id).to_string(),
    Some(Json(task)))
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