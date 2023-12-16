//#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod controllers;
mod models;
mod services;
mod connection;
mod schema;

fn main() {
    dotenv().ok();
    controllers::routers::create_routes();
}

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
//         .mount("/ingredients", routes![controllers::ingredients_controller::get_ingredients,
//                                        controllers::ingredients_controller::add_new_ingredient,
//                                        controllers::ingredients_controller::modify_ingredient,
//                                        controllers::ingredients_controller::delete_ingredient])
// }

