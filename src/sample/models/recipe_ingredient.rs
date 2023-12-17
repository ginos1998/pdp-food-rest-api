#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::recipe_ingredient;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "recipe_ingredient"]
pub struct Recipe {
    pub id_recipe_ingredient: i32,   // pk
    pub id_recipe: i32,   
    pub id_ingredient: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="recipe_ingredient"]
pub struct RecipeDTO {
    pub id_recipe: i32,
    pub id_ingredient: i32,
}