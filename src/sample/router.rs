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
        )
        .mount("/ingredients", 
               routes![
                    sample::controllers::ingredient_controller::all_ingredients,
                    sample::controllers::ingredient_controller::create_ingredient,
                    sample::controllers::ingredient_controller::get_ingredient_by_id,
                    sample::controllers::ingredient_controller::update_ingredient_by_id,
                    sample::controllers::ingredient_controller::delete_ingredient_by_id
                    ], )
        .launch();
}