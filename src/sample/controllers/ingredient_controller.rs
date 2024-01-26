use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::services::ingredient_service;
use crate::sample::models::ingredient::{Ingredient, IngredientDTO};

#[get("/?<limit>")]
pub fn all_ingredients(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Ingredient>>, Status> {
    ingredient_service::all_ingredients(limit, connection)
}

#[post("/", format ="application/json", data = "<new_ingredient>")]
pub fn create_ingredient(new_ingredient: Json<IngredientDTO>, connection: DbConn) ->  Result<status::Created<Json<Ingredient>>, Status> {
    ingredient_service::create_ingredient(new_ingredient.into_inner(), connection)

}

#[get("/<id>")]
pub fn get_ingredient_by_id(id: i32, connection: DbConn) -> Result<Json<Ingredient>, Status> {
    ingredient_service::get_ingredient_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<ingredient_updated>")]
pub fn update_ingredient_by_id(id: i32, ingredient_updated: Json<Ingredient>, connection: DbConn) -> Result<Json<Ingredient>, Status> {
    ingredient_service::update_ingredient_by_id(id, ingredient_updated.into_inner(), connection)
}

#[delete("/<id>")]
pub fn delete_ingredient_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    ingredient_service::delete_ingredient_by_id(id, connection)   
}

#[get("/recipe/<recipe_id>")]
pub fn get_ingredient_by_recipe(recipe_id: i32, connection: DbConn) -> Result<Json<Vec<Ingredient>>, Status> {
    ingredient_service::get_ingredient_by_recipe(recipe_id, connection)
}