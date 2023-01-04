use rocket;

use crate::connection;
use crate::route;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts",
               routes![
                route::handler::all_posts,
                route::handler::create_post,
                route::handler::get_post,
                route::handler::update_post,
                route::handler::delete_post
                    ],
        ).launch();
}