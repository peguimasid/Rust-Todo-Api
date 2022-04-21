use super::schema::todos;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "todos"]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub done: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
  pub title: String,
}