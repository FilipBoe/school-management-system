use rocket::http::Method;
use rocket_cors::{ AllowedOrigins, CorsOptions };

pub fn create_cors() -> CorsOptions {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch].into_iter().map(From::from).collect()
        )
        .allow_credentials(true)
}
