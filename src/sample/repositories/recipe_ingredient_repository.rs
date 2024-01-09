#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::recipe_ingredient::{RecipeIngredientDTO, RecipeIngredient};

use crate::schema::recipe_ingredient;

pub fn create_recipe_ingredient(new_recipe: RecipeIngredientDTO, conn: &PgConnection) -> QueryResult<RecipeIngredient> {
    diesel::insert_into(recipe_ingredient::table)
        .values(&new_recipe)
        .get_result(conn)
}