use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::controllers;
use crate::models::ingredient::Ingredient;
use crate::services::ingredients_service;

#[get("/")]
pub fn get_ingredients(connection: DbConn) -> Result<Json<Vec<Ingredient>>, Status> {
    controllers::ingredients_repository::show_ingredients(&connection)
        .map(|ingredient| Json(ingredient))
        .map_err(|error| error_status(error))

    // let ing = ingredients_service::get_ingredients();
    // Json(ing)
}

// #[post("/", format ="application/json", data = "<new_post>")]
// pub fn add_new_ingredient() -> &'static str {
//     "agregando ingrediente"
// }
//
// #[put("/")]
// pub fn modify_ingredient() -> &'static str {
//     "modificando ingrediente"
// }
//
// #[delete("/")]
// pub fn delete_ingredient() -> &'static str {
//     "eliminando ingrediente"
// }

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}