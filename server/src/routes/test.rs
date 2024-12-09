use rocket::serde::json::Json;

#[get("/test")]
pub fn test() -> Json<&'static str> {
    Json("Hello, world!")
}
