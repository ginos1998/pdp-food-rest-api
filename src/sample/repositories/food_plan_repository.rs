#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::food_plan::{FoodPlan, FoodPlanDTO};

use crate::schema::food_plan;
use crate::schema::food_plan::dsl::*;

pub fn create_plan(new_plan: FoodPlanDTO, conn: &PgConnection) -> QueryResult<FoodPlan> {
    diesel::insert_into(food_plan::table)
        .values(&new_plan)
        .get_result(conn)
}

pub fn show_plans(limit: i64, connection: &PgConnection) -> QueryResult<Vec<FoodPlan>>  {
    food_plan.limit(limit)
        .load::<FoodPlan>(&*connection)
}

pub fn get_plan_by_id(plan_id: i32, connection: &PgConnection) -> QueryResult<FoodPlan> {
    food_plan::table.find(plan_id).get_result::<FoodPlan>(connection)
}

pub fn update_plan_by_id(plan_id: i32, updated_plan: FoodPlan, connection: &PgConnection) -> QueryResult<FoodPlan> {
    diesel::update(food_plan::table.find(plan_id))
        .set(&updated_plan)
        .get_result(connection)
}

pub fn delete_plan_by_id(plan_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(food_plan::table.find(plan_id))
        .execute(connection)
}

