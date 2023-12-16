use rocket;

use crate::connection;
use crate::sample;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts",
               routes![
                    sample::controllers::posts_controller::all_posts,
                    sample::controllers::posts_controller::create_post,
                    sample::controllers::posts_controller::get_post,
                    sample::controllers::posts_controller::update_post,
                    sample::controllers::posts_controller::delete_post
                    ],
        ).launch();
}