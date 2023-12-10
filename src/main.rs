mod controllers;
mod models;
mod services;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/ingredients", routes![controllers::ingredients_controller::get_ingredients,
                                       controllers::ingredients_controller::add_new_ingredient,
                                       controllers::ingredients_controller::modify_ingredient,
                                       controllers::ingredients_controller::delete_ingredient])
}

