use crate::connection;
use crate::handler;
use rocket;

pub fn create_routes() {
  rocket::ignite()
    .manage(connection::init_pool())
    .mount(
      "/tasks",
      routes![
        handler::all_tasks,
        handler::pending_tasks,
        handler::create_task,
        handler::get_task,
        handler::update_task,
        handler::done_task,
        handler::delete_task
      ],
    )
    .launch();
}
