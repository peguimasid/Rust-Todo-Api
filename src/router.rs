use crate::connection;
use crate::handler;
use rocket;

pub fn create_routes() {
  rocket::ignite()
    .manage(connection::init_pool())
    .mount(
      "/todos",
      routes![
        handler::all_todos,
        handler::pending_todos,
        handler::create_todo,
        handler::get_todo,
        handler::update_todo,
        handler::done_todo,
        handler::delete_todo
      ],
    )
    .launch();
}
