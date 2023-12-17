#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::food_plan_recipe;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "food_plan_recipe"]
pub struct FoodPlanRecipe {
    pub id_food_plan_recipe: i32, // pk
    pub id_recipe: i32,
    pub id_food_plan: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="food_plan_recipe"]
pub struct FoodPlanRecipeDTO {
    pub id_recipe: i32,
    pub id_food_plan: i32,
}