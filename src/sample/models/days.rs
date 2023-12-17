#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::day;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "day"]
pub struct Day {
    pub id_day: i32,    // pk
    pub day_name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="day"]
pub struct DayDTO {
    pub day_name: String,
}