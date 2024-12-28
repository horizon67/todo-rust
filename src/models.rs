use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::todo_tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoTask {
    pub id: i32,
    pub content: String,
    pub state: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::todo_tasks)]
pub struct NewTodoTask {
    pub content: String,
    pub state: Option<i32>,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::todo_tasks)]
pub struct UpdateTodoTask {
    pub content: Option<String>,
    pub state: Option<i32>,
}
