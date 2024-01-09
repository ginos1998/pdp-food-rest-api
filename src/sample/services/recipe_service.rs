use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::{recipe_repository, recipe_ingredient_repository};
use crate::sample::models::recipe::{Recipe, RecipeDTO, RecipeIngredientCategoryDTO};
use crate::sample::models::recipe_ingredient::RecipeIngredientDTO;
use crate::sample::exceptions::errors::error_status;
use crate::sample::utils::common_functions::get_limit_or_default;

pub fn all_recipes(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Recipe>>, Status> {
    let aux_limit = get_limit_or_default(limit, 5);
    recipe_repository::show_recipes(aux_limit, &connection)
        .map(|recipe| Json(recipe))
        .map_err(|error| error_status(error))
}

pub fn create_recipe(new_recipe: RecipeDTO, connection: DbConn) ->  Result<status::Created<Json<Recipe>>, Status> {
    recipe_repository::create_recipe(new_recipe, &connection)
    .map(|recipe| recipe_created(recipe))
    .map_err(|error| error_status(error))
}

pub fn create_recipe_ingredient(recipe_ingredient_dto: RecipeIngredientCategoryDTO, connection: DbConn) ->  Result<status::Created<Json<Recipe>>, Status> {
    let recipe_dto = RecipeDTO {
        recipe_name: recipe_ingredient_dto.recipe_name,
        id_category: recipe_ingredient_dto.id_category,
    };

    match recipe_repository::create_recipe(recipe_dto, &connection) {
        Ok(inserted_recipe) => {
            let new_id = inserted_recipe.id_recipe;
            println!("Nueva receta creada con ID: {}", new_id);
            for ingredient_id in recipe_ingredient_dto.ingredient_ids{
                let recipe_ingredient_dto = RecipeIngredientDTO {
                    id_recipe: new_id,
                    id_ingredient: ingredient_id
                };
                if let Err(error) = recipe_ingredient_repository::create_recipe_ingredient(recipe_ingredient_dto, &connection) {
                    print!("Error al insertar en recipe_ingredient");
                    return Err(error_status(error));
                }
            }
            Ok(recipe_created(inserted_recipe))
        }
        Err(error) => Err(error_status(error)),
    }  

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