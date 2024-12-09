use diesel::prelude::*;

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}

pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub user_id: i32,
}
