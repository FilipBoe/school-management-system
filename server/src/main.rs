use rocket::fs::FileServer;

mod config;
mod cors;
mod routes;
mod clerk;

use config::Config;
use clerk::load;
use cors::create_cors;
use routes::users::get_users;
use routes::test::test;
use routes::frontend::index;
use routes::posts::get_posts;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let setup_config = Config::load();

    let clerk_guard_config = load(&setup_config);

    rocket
        ::build()
        .attach(create_cors().to_cors().unwrap())
        .manage(clerk_guard_config)
        .manage(setup_config.clone())
        .mount("/assets", FileServer::from(setup_config.asset_path()).rank(1))
        .mount("/api/v1", routes![get_users, test, get_posts])
        .mount("/", routes![index])
}
