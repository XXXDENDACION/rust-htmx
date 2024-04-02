use crate::schema::todo;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::todo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub pos: i32,
}

#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub pos: &'a i32,
}