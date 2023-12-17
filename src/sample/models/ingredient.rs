#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::ingredient;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "ingredient"]
pub struct Ingredient {
    pub id_ingredient: i32, // pk
    pub ingredient_name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="ingredient"]
pub struct IngredientDTO {
    pub ingredient_name: String,
}
