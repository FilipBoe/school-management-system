use rocket::serde::json::Json;
use rust_backend::database::establish_connection;
use rust_backend::models::User;
use diesel::prelude::*;

#[get("/posts")]
pub fn get_posts() -> Json<Vec<User>> {
    use rust_backend::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let result: Vec<User> = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    Json(result)
}
