use super::schema::tasks;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "tasks"]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub done: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
  pub title: String,
}