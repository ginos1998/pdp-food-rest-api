use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::ingredient_repository;
use crate::sample::models::ingredient::Ingredient;
use crate::sample::models::ingredient::IngredientDTO;
use crate::sample::exceptions::errors::error_status;
use crate::sample::utils::common_functions::get_limit_or_default;

pub fn all_ingredients(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Ingredient>>, Status> {
    let aux_limit = get_limit_or_default(limit, 20);
    ingredient_repository::show_ingredients(aux_limit, &connection)
        .map(|ingredient| Json(ingredient))
        .map_err(|error| error_status(error))
}

pub fn create_ingredient(new_ingredient: IngredientDTO, connection: DbConn) ->  Result<status::Created<Json<Ingredient>>, Status> {
    ingredient_repository::create_ingredient(new_ingredient, &connection)
    .map(|ingredient| ingredient_created(ingredient))
    .map_err(|error| error_status(error))
}

pub fn get_ingredient_by_id(id: i32, connection: DbConn) -> Result<Json<Ingredient>, Status> {
    ingredient_repository::get_ingredient_by_id(id, &connection)
        .map(|ingredient| Json(ingredient))
        .map_err(|error| error_status(error))
}

pub fn update_ingredient_by_id(id: i32, ingredient_updated: Ingredient, connection: DbConn) -> Result<Json<Ingredient>, Status> {
    ingredient_repository::update_ingredient_by_id(id, ingredient_updated, &connection)
        .map(|ingredient| Json(ingredient))
        .map_err(|error| error_status(error))
}

pub fn delete_ingredient_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    ingredient_repository::delete_ingredient_by_id(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn ingredient_created(ing: Ingredient) -> status::Created<Json<Ingredient>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/ingredients/{id}", host = host(), port = port(), id = ing.id_ingredient).to_string(),
        Some(Json(ing)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}