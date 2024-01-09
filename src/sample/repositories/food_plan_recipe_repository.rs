#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::food_plan_recipe::{FoodPlanRecipeDTO, FoodPlanRecipe};

use crate::schema::food_plan_recipe;

pub fn create_food_plan_receipe(new_food_plan_receipe: FoodPlanRecipeDTO, conn: &PgConnection) -> QueryResult<FoodPlanRecipe> {
    diesel::insert_into(food_plan_recipe::table)
        .values(&new_food_plan_receipe)
        .get_result(conn)
}