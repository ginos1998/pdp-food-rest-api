#![allow(proc_macro_derive_resolution_fallback)]

//use crate::models::recipe;
use crate::schema::recipe;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "recipe"]
pub struct Recipe {
    pub recipe_id: i32,
    pub name: String
}