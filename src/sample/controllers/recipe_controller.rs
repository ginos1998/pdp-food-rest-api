use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::services::recipe_service;
use crate::sample::models::recipe::{Recipe, RecipeDTO, RecipeIngredientCategoryDTO};

#[get("/?<limit>")]
pub fn all_recipes(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Recipe>>, Status> {
    recipe_service::all_recipes(limit, connection)
}

#[post("/", format ="application/json", data = "<new_recipe>")]
pub fn create_recipe(new_recipe: Json<RecipeDTO>, connection: DbConn) ->  Result<status::Created<Json<Recipe>>, Status> {
    recipe_service::create_recipe(new_recipe.into_inner(), connection)

}

#[post("/custom", format ="application/json", data = "<new_recipe>")]
pub fn create_custom_recipe(new_recipe: Json<RecipeIngredientCategoryDTO>, connection: DbConn) ->  Result<status::Created<Json<Recipe>>, Status> {
    recipe_service::create_recipe_ingredient(new_recipe.into_inner(), connection)

}

#[get("/<id>")]
pub fn get_recipe_by_id(id: i32, connection: DbConn) -> Result<Json<Recipe>, Status> {
    recipe_service::get_recipe_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<recipe_updated>")]
pub fn update_recipe_by_id(id: i32, recipe_updated: Json<Recipe>, connection: DbConn) -> Result<Json<Recipe>, Status> {
    recipe_service::update_recipe_by_id(id, recipe_updated.into_inner(), connection)
}

#[delete("/<id>")]
pub fn delete_recipe_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    recipe_service::delete_recipe_by_id(id, connection)   
}

#[get("/category/<category_id>")]
pub fn get_recipe_by_category(category_id: i32, connection: DbConn) -> Result<Json<Vec<Recipe>>, Status> {
    recipe_service::get_recipe_by_category(category_id, connection)
}

#[get("/plan/<plan_id>")]
pub fn get_recipe_by_plan(plan_id: i32, connection: DbConn) -> Result<Json<Vec<Recipe>>, Status> {
    recipe_service::get_recipe_by_plan(plan_id, connection)
}
