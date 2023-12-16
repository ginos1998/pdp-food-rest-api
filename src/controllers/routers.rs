use rocket;

use crate::connection;
use crate::controllers;

pub fn create_routes() {
    rocket::build()
        .manage(connection::init_pool())
        .mount("/ingredients",
               routes![
                   controllers::ingredients_controller::get_ingredients
                   // controllers::ingredients_controller::add_new_ingredient,
                   // controllers::ingredients_controller::modify_ingredient,
                   // controllers::ingredients_controller::delete_ingredient
               ],
        ).launch();
}