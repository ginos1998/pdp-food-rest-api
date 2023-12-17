#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::food_plan_day;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "food_plan_day"]
pub struct FoodPlanDay {
    pub id_food_plan_day: i32, // pk
    pub id_food_plan: i32,
    pub id_day: i32,
    pub id_menu: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="food_plan_day"]
pub struct FoodPlanDayDTO {
    pub id_food_plan: i32,
    pub id_day: i32,
    pub id_menu: i32,
}