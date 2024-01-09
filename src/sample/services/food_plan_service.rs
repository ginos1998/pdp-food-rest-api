use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::{food_plan_repository, food_plan_recipe_repository};
use crate::sample::models::food_plan::{FoodPlan, FoodPlanDTO, PlanRecipeDTO};
use crate::sample::models::food_plan_recipe::FoodPlanRecipeDTO;

use crate::sample::exceptions::errors::error_status;
use crate::sample::utils::common_functions::get_limit_or_default;

pub fn all_plans(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<FoodPlan>>, Status> {
    let aux_limit = get_limit_or_default(limit, 5);
    food_plan_repository::show_plans(aux_limit, &connection)
        .map(|plan| Json(plan))
        .map_err(|error| error_status(error))
}

pub fn create_plan(new_plan: FoodPlanDTO, connection: DbConn) ->  Result<status::Created<Json<FoodPlan>>, Status> {
    food_plan_repository::create_plan(new_plan, &connection)
    .map(|plan| plan_created(plan))
    .map_err(|error| error_status(error))
}

pub fn create_custom_plan(new_plan: PlanRecipeDTO, connection: DbConn) ->  Result<status::Created<Json<FoodPlan>>, Status> {
    let plan_dto = FoodPlanDTO {
        food_plan_name: new_plan.food_plan_name,
        description: new_plan.description,
    };

    match food_plan_repository::create_plan(plan_dto, &connection) {
        Ok(inserted_plan) => {
            let new_id = inserted_plan.id_food_plan;
            println!("Nueva receta creada con ID: {}", new_id);
            for recipe_id in new_plan.id_recipe{
                let plan_recipe_dto = FoodPlanRecipeDTO {
                    id_recipe: recipe_id,
                    id_food_plan: new_id
                };
                if let Err(error) = food_plan_recipe_repository::create_food_plan_receipe(plan_recipe_dto, &connection) {
                    print!("Error al insertar en food_plan_recipe");
                    return Err(error_status(error));
                }
            }
            Ok(plan_created(inserted_plan))
        }
        Err(error) => Err(error_status(error)),
    }
    
}

pub fn get_plan_by_id(id: i32, connection: DbConn) -> Result<Json<FoodPlan>, Status> {
    food_plan_repository::get_plan_by_id(id, &connection)
        .map(|plan| Json(plan))
        .map_err(|error| error_status(error))
}

pub fn update_plan_by_id(id: i32, plan_updated: FoodPlan, connection: DbConn) -> Result<Json<FoodPlan>, Status> {
    food_plan_repository::update_plan_by_id(id, plan_updated, &connection)
        .map(|plan| Json(plan))
        .map_err(|error| error_status(error))
}

pub fn delete_plan_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    food_plan_repository::delete_plan_by_id(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn plan_created(plan: FoodPlan) -> status::Created<Json<FoodPlan>> {
    status::Created(
        format!("{host}:{port}/plans/{id}", host = host(), port = port(), id = plan.id_food_plan).to_string(),
        Some(Json(plan)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}


