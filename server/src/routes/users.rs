use rocket::serde::json::Json;
use rust_backend::database::establish_connection;
use rust_backend::models::User;
use diesel::prelude::*;
use clerk_rs::validators::jwks::MemoryCacheJwksProvider;
use clerk_rs::validators::rocket::ClerkGuard;

#[get("/users")]
pub fn get_users(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> Json<Vec<User>> {
    use rust_backend::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let result: Vec<User> = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    Json(result)
}
