use rocket::get;
use rocket::post;
use rocket::put;
use rocket::delete;

use rocket::serde::json::Json;
use crate::models::ingredient::Ingredient;

use crate::services::ingredients_service;

#[get("/")]
pub fn get_ingredients() -> Json<Ingredient> {
    let ing = ingredients_service::get_ingredients();
    Json(ing)
}

#[post("/")]
pub fn add_new_ingredient() -> &'static str {
    "agregando ingrediente"
}

#[put("/")]
pub fn modify_ingredient() -> &'static str {
    "modificando ingrediente"
}

#[delete("/")]
pub fn delete_ingredient() -> &'static str {
    "eliminando ingrediente"
}