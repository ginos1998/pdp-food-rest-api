#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::category::Category;
use crate::sample::models::category::CategoryDTO;

use crate::schema::category;
use crate::schema::category::dsl::*;

pub fn create_category(new_category: CategoryDTO, conn: &PgConnection) -> QueryResult<Category> {
    diesel::insert_into(category::table)
        .values(&new_category)
        .get_result(conn)
}

pub fn show_categories(limit: i64, connection: &PgConnection) -> QueryResult<Vec<Category>>  {
    category.limit(limit)
        .load::<Category>(&*connection)
}

pub fn get_category_by_id(category_id: i32, connection: &PgConnection) -> QueryResult<Category> {
    category::table.find(category_id).get_result::<Category>(connection)
}

pub fn update_category_by_id(category_id: i32, updated_ingredient: Category, connection: &PgConnection) -> QueryResult<Category> {
    diesel::update(category::table.find(category_id))
        .set(&updated_ingredient)
        .get_result(connection)
}

pub fn delete_category_by_id(category_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(category::table.find(category_id))
        .execute(connection)
}
