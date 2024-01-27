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

pub fn delete_recipe_ingredient(id_recipe: i32, id_ingredient: i32, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(
        recipe_ingredient::table.filter(
                recipe_ingredient::id_recipe.eq(id_recipe)
                .and(recipe_ingredient::id_ingredient.eq(id_ingredient))
            )
        )
        .execute(conn)
}