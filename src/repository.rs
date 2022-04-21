#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model::Task;
use crate::model::NewTask;

use crate::schema::tasks;
use crate::schema::tasks::dsl::*;

pub fn show_all_tasks(connection: &PgConnection) -> QueryResult<Vec<Task>>  {
  tasks.load::<Task>(&*connection)
}

pub fn show_pending_tasks(connection: &PgConnection) -> QueryResult<Vec<Task>>  {
  tasks.filter(done.eq(false)).load::<Task>(&*connection)
}

pub fn get_task(task_id: i32, connection: &PgConnection) -> QueryResult<Task> {
  tasks::table.find(task_id).get_result::<Task>(connection)
}

pub fn create_task(new_task: NewTask, conn: &PgConnection) -> QueryResult<Task> {
  diesel::insert_into(tasks::table).values(&new_task).get_result(conn)
}

pub fn update_task(task_id: i32, task: Task, connection: &PgConnection) -> QueryResult<Task> {
  diesel::update(tasks::table.find(task_id)).set(&task).get_result(connection)
}

pub fn done_task(task_id: i32, connection: &PgConnection) -> QueryResult<Task> {
  diesel::update(tasks::table.find(task_id)).set(done.eq(true)).get_result(connection)
}

pub fn delete_task(task_id: i32, connection: &PgConnection) -> QueryResult<usize> {
  diesel::delete(tasks::table.find(task_id)).execute(connection)
}
