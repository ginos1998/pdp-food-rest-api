#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::category;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "category"]
pub struct Category {
    pub id_category: i32,   // pk
    pub category_name: String,
    pub active : bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="category"]
pub struct CategoryDTO {
    pub category_name: String,
    pub active : bool,
}