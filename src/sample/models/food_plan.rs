#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::food_plan;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "food_plan"]
pub struct FoodPlan {
    pub id_food_plan: i32, // pk
    pub food_plan_name: String,
    pub description: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="food_plan"]
pub struct FoodPlanDTO {
    pub food_plan_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanRecipeDTO {
    pub food_plan_name: String,
    pub description: String,
    pub id_recipe: Vec<i32>,
}