#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::models::menu::Menu;
use crate::sample::models::menu::MenuDTO;

use crate::schema::menu;
use crate::schema::menu::dsl::*;

pub fn create_menu(new_menu: MenuDTO, conn: &PgConnection) -> QueryResult<Menu> {
    diesel::insert_into(menu::table)
        .values(&new_menu)
        .get_result(conn)
}

pub fn show_menues(connection: &PgConnection) -> QueryResult<Vec<Menu>>  {
    //posts.filter(published.eq(true))
    menu.limit(5)
        .load::<Menu>(&*connection)
}

pub fn get_menu_by_id(menu_id: i32, connection: &PgConnection) -> QueryResult<Menu> {
    menu::table.find(menu_id).get_result::<Menu>(connection)
}

pub fn update_menu_by_id(menu_id: i32, updated_ingredient: Menu, connection: &PgConnection) -> QueryResult<Menu> {
    diesel::update(menu::table.find(menu_id))
        .set(&updated_ingredient)
        .get_result(connection)
}

pub fn delete_menu_by_id(menu_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(menu::table.find(menu_id))
        .execute(connection)
}
