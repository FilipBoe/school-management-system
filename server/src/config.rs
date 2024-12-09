use std::env;
use dotenvy::dotenv;

#[derive(Clone)]
pub struct Config {
    pub frontend_path: String,
    pub clerk_secret_key: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            frontend_path: env::var("FRONTEND_PATH").expect("FRONTEND_PATH must be set"),
            clerk_secret_key: env::var("CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY must be set"),
        }
    }

    pub fn asset_path(&self) -> String {
        format!("{}/assets", self.frontend_path)
    }

    pub fn index_path(&self) -> String {
        format!("{}/index.html", self.frontend_path)
    }
}
