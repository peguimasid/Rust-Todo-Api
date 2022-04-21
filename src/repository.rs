#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::model::Todo;
use crate::sample::model::NewTodo;

use crate::schema::todos;
use crate::schema::todos::dsl::*;

pub fn create_todo(new_todo: NewTodo, conn: &PgConnection) -> QueryResult<Todo> {
  diesel::insert_into(todos::table).values(&new_todo).get_result(conn)
}

pub fn show_todos(connection: &PgConnection) -> QueryResult<Vec<Todo>>  {
  // tod os.filter(done.eq(false))
  todos.limit(5).load::<Todo>(&*connection)
}

pub fn get_todo(todo_id: i32, connection: &PgConnection) -> QueryResult<Todo> {
  posts::table.find(todo_id).get_result::<Post>(connection)
}

pub fn update_todo(todo_id: i32, todo: Todo, connection: &PgConnection) -> QueryResult<Todo> {
  diesel::update(todos::table.find(todo_id)).set(&todo).get_result(connection)
}

pub fn delete_todo(todo_id: i32, connection: &PgConnection) -> QueryResult<usize> {
  diesel::delete(todos::table.find(todo_id)).execute(connection)
}
