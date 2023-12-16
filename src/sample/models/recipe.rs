#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::recipe;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "recipe"]
pub struct Recipe {
    pub id_recipe: i32,   // pk
    pub recipe_name: String,
    pub id_category: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="recipe"]
pub struct RecipeDTO {
    pub recipe_name: String,
    pub id_category: i32,
}