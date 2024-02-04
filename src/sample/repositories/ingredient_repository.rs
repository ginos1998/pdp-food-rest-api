#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::ingredient::Ingredient;
use crate::sample::models::ingredient::IngredientDTO;

use crate::schema::ingredient;
use crate::schema::ingredient::dsl::*;
use crate::schema::recipe_ingredient;
use crate::schema::recipe_ingredient::dsl::id_recipe as ri_recipe;

pub fn create_ingredient(new_ingredient: IngredientDTO, conn: &PgConnection) -> QueryResult<Ingredient> {
    diesel::insert_into(ingredient::table)
        .values(&new_ingredient)
        .get_result(conn)
}

pub fn show_ingredients(limit: i64, connection: &PgConnection) -> QueryResult<Vec<Ingredient>>  {
    ingredient.limit(limit)
        .load::<Ingredient>(&*connection)
}

/**
 * Get ingredient by id
 * @param id_ing id of ingredient
 * @param connection connection to database
 * @return ingredient
 */
pub fn get_ingredient_by_id(ingredient_id: i32, connection: &PgConnection) -> QueryResult<Ingredient> {
    ingredient::table.find(ingredient_id)
    .get_result::<Ingredient>(connection)
}

pub fn update_ingredient_by_id(ingredient_id: i32, updated_ingredient: Ingredient, connection: &PgConnection) -> QueryResult<Ingredient> {
    diesel::update(ingredient::table.find(ingredient_id))
        .set(&updated_ingredient)
        .get_result(connection)
}

pub fn delete_ingredient_by_id(ingredient_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(ingredient::table.find(ingredient_id))
        .execute(connection)
}


pub fn get_ingredient_by_recipe(recipe_id: i32, connection: &PgConnection) -> QueryResult<Vec<Ingredient>> {
    recipe_ingredient::table.filter(ri_recipe.eq(recipe_id))
        .inner_join(ingredient::table)
        .select(ingredient::all_columns)
        .load::<Ingredient>(connection)
}