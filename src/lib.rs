#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod catchers;
mod config;
mod db_connection;
mod domains;
mod from_requests;
mod handlers;
mod helpers;
mod schema;
mod test_helper;

pub fn rocket_builder() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api",
            routes![
                handlers::healthz_handler::index,
                handlers::member_handler::get_request,
                handlers::member_handler::get_member_request,
                handlers::member_handler::create_request,
                handlers::member_handler::update_request,
                handlers::session_handler::create_request,
                handlers::session_handler::auth_request,
                handlers::post_handler::create_post_request,
                handlers::post_handler::vote_post_request,
            ],
        )
        .register(
            "/api/members",
            catchers![catchers::auth_catcher::member_not_found,],
        )
        .register(
            "/api",
            catchers![catchers::bad_request_catcher::bad_request_field,],
        )
        .manage(db_connection::establish_connection())
}
