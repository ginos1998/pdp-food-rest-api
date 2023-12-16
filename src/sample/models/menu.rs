#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::menu;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "menu"]
pub struct Menu {
    pub id_menu: i32,   // pk
    pub menu_name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="menu"]
pub struct MenuDTO {
    pub menu_name: String,
}