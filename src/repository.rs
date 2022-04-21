#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model::Todo;
use crate::model::NewTodo;

use crate::schema::todos;
use crate::schema::todos::dsl::*;

pub fn show_all_todos(connection: &PgConnection) -> QueryResult<Vec<Todo>>  {
  todos.load::<Todo>(&*connection)
}

pub fn show_pending_todos(connection: &PgConnection) -> QueryResult<Vec<Todo>>  {
  todos.filter(done.eq(false)).load::<Todo>(&*connection)
}

pub fn get_todo(todo_id: i32, connection: &PgConnection) -> QueryResult<Todo> {
  todos::table.find(todo_id).get_result::<Todo>(connection)
}

pub fn create_todo(new_todo: NewTodo, conn: &PgConnection) -> QueryResult<Todo> {
  diesel::insert_into(todos::table).values(&new_todo).get_result(conn)
}

pub fn update_todo(todo_id: i32, todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
  diesel::update(todos::table.find(todo_id)).set(&todo).get_result(connection)
}

pub fn done_todo(todo_id: i32, connection: &PgConnection) -> QueryResult<Todo> {
  diesel::update(todos::table.find(todo_id)).set(done.eq(true)).get_result(connection)
}

pub fn delete_todo(todo_id: i32, connection: &PgConnection) -> QueryResult<usize> {
  diesel::delete(todos::table.find(todo_id)).execute(connection)
}
