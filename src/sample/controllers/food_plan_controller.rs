use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;

use crate::sample::models::food_plan::{FoodPlan, FoodPlanDTO, PlanRecipeDTO};
use crate::sample::services::food_plan_service;

#[get("/?<limit>")]
pub fn all_plans(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<FoodPlan>>, Status> {
    food_plan_service::all_plans(limit, connection)
}

#[post("/", format ="application/json", data = "<new_recipe>")]
pub fn create_plan(new_recipe: Json<FoodPlanDTO>, connection: DbConn) ->  Result<status::Created<Json<FoodPlan>>, Status> {
    food_plan_service::create_plan(new_recipe.into_inner(), connection)

}

#[post("/custom", format ="application/json", data = "<new_plan>")]
pub fn create_custom_plan(new_plan: Json<PlanRecipeDTO>, connection: DbConn) ->  Result<status::Created<Json<FoodPlan>>, Status> {
    food_plan_service::create_custom_plan(new_plan.into_inner(), connection)
}

#[get("/<id>")]
pub fn get_plan_by_id(id: i32, connection: DbConn) -> Result<Json<FoodPlan>, Status> {
    food_plan_service::get_plan_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<plan_updated>")]
pub fn update_plan_by_id(id: i32, plan_updated: Json<PlanRecipeDTO>, connection: DbConn) -> Result<Json<FoodPlan>, Status> {
    food_plan_service::update_plan_by_id(id, plan_updated.into_inner(), connection)
}

#[put("/<id>/delete-recipe/<id_recipe>")]
pub fn delete_recipe_from_plan(id: i32, id_recipe: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    food_plan_service::delete_recipe_from_plan(id, id_recipe, connection)
}

#[delete("/<id>")]
pub fn delete_plan_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    food_plan_service::delete_plan_by_id(id, connection)   
}