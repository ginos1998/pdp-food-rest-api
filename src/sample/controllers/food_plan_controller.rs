use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;

use crate::sample::models::food_plan::{FoodPlan, FoodPlanDTO};
use crate::sample::services::food_plan_service;

#[get("/?<limit>")]
pub fn all_plans(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<FoodPlan>>, Status> {
    food_plan_service::all_plans(limit, connection)
}

#[post("/", format ="application/json", data = "<new_recipe>")]
pub fn create_plan(new_recipe: Json<FoodPlanDTO>, connection: DbConn) ->  Result<status::Created<Json<FoodPlan>>, Status> {
    food_plan_service::create_plan(new_recipe.into_inner(), connection)

}

#[get("/<id>")]
pub fn get_plan_by_id(id: i32, connection: DbConn) -> Result<Json<FoodPlan>, Status> {
    food_plan_service::get_plan_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<recipe_updated>")]
pub fn update_plan_by_id(id: i32, recipe_updated: Json<FoodPlan>, connection: DbConn) -> Result<Json<FoodPlan>, Status> {
    food_plan_service::update_plan_by_id(id, recipe_updated.into_inner(), connection)
}

#[delete("/<id>")]
pub fn delete_plan_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    food_plan_service::delete_plan_by_id(id, connection)   
}