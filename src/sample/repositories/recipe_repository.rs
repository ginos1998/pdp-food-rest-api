#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::recipe::{Recipe, RecipeDTO};

use crate::schema::recipe;
use crate::schema::recipe::dsl::*;

use crate::schema::food_plan_recipe;
use crate::schema::food_plan_recipe::dsl::id_food_plan as fpr_id_food_plan;

pub fn create_recipe(new_recipe: RecipeDTO, conn: &PgConnection) -> QueryResult<Recipe> {
    diesel::insert_into(recipe::table)
        .values(&new_recipe)
        .get_result(conn)
}

pub fn show_recipes(limit: i64, connection: &PgConnection) -> QueryResult<Vec<Recipe>>  {
    recipe.limit(limit)
        .load::<Recipe>(&*connection)
}

pub fn get_recipe_by_id(recipe_id: i32, connection: &PgConnection) -> QueryResult<Recipe> {
    recipe::table.find(recipe_id).get_result::<Recipe>(connection)
}

pub fn update_recipe_by_id(recipe_id: i32, updated_ingredient: Recipe, connection: &PgConnection) -> QueryResult<Recipe> {
    diesel::update(recipe::table.find(recipe_id))
        .set(&updated_ingredient)
        .get_result(connection)
}

pub fn delete_recipe_by_id(recipe_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(recipe::table.find(recipe_id))
        .execute(connection)
}

pub fn get_recipe_by_category(category_id: i32, connection: &PgConnection) -> QueryResult<Vec<Recipe>> {
    recipe::table.filter(id_category.eq(category_id))
        .load::<Recipe>(connection)
}

pub fn get_recipe_by_plan(plan_id: i32, connection: &PgConnection) -> QueryResult<Vec<Recipe>> {
    food_plan_recipe::table.filter(fpr_id_food_plan.eq(plan_id))
        .inner_join(recipe::table)
        .select(recipe::all_columns)
        .load::<Recipe>(connection)
}