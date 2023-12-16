#![allow(proc_macro_derive_resolution_fallback)]

//use crate::models::ingredient;
use crate::schema::ingredient;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "ingredient"]
pub struct Ingredient {
    pub ingredient_id: i32,
    pub name: String,
}
