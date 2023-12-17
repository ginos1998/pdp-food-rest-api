use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::recipe_repository;
use crate::sample::models::recipe::Recipe;
use crate::sample::models::recipe::RecipeDTO;
use crate::sample::exceptions::errors::error_status;

pub fn all_recipes(connection: DbConn) -> Result<Json<Vec<Recipe>>, Status> {
    recipe_repository::show_recipes(&connection)
        .map(|recipe| Json(recipe))
        .map_err(|error| error_status(error))
}

pub fn create_recipe(new_recipe: RecipeDTO, connection: DbConn) ->  Result<status::Created<Json<Recipe>>, Status> {
    recipe_repository::create_recipe(new_recipe, &connection)
    .map(|recipe| recipe_created(recipe))
    .map_err(|error| error_status(error))
}

pub fn get_recipe_by_id(id: i32, connection: DbConn) -> Result<Json<Recipe>, Status> {
    recipe_repository::get_recipe_by_id(id, &connection)
        .map(|recipe| Json(recipe))
        .map_err(|error| error_status(error))
}

pub fn update_recipe_by_id(id: i32, recipe_updated: Recipe, connection: DbConn) -> Result<Json<Recipe>, Status> {
    recipe_repository::update_recipe_by_id(id, recipe_updated, &connection)
        .map(|recipe| Json(recipe))
        .map_err(|error| error_status(error))
}

pub fn delete_recipe_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    recipe_repository::delete_recipe_by_id(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn recipe_created(ing: Recipe) -> status::Created<Json<Recipe>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/recipe/{id}", host = host(), port = port(), id = ing.id_recipe).to_string(),
        Some(Json(ing)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}