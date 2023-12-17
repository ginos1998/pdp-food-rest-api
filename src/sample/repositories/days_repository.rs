#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::days::Day;
use crate::sample::models::days::DayDTO;

use crate::schema::day;
use crate::schema::day::dsl::*;

pub fn create_day(new_day: DayDTO, conn: &PgConnection) -> QueryResult<Day> {
    diesel::insert_into(day::table)
        .values(&new_day)
        .get_result(conn)
}

pub fn show_days(limit: i64, connection: &PgConnection) -> QueryResult<Vec<Day>>  {
    day.limit(limit)
        .load::<Day>(&*connection)
}

pub fn get_day_by_id(day_id: i32, connection: &PgConnection) -> QueryResult<Day> {
    day::table.find(day_id).get_result::<Day>(connection)
}

pub fn update_day_by_id(day_id: i32, updated_ingredient: Day, connection: &PgConnection) -> QueryResult<Day> {
    diesel::update(day::table.find(day_id))
        .set(&updated_ingredient)
        .get_result(connection)
}

pub fn delete_day_by_id(day_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(day::table.find(day_id))
        .execute(connection)
}
