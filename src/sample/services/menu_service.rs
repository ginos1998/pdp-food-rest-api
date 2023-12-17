use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::menu_repository;
use crate::sample::models::menu::Menu;
use crate::sample::models::menu::MenuDTO;
use crate::sample::exceptions::errors::error_status;
use crate::sample::utils::common_functions::get_limit_or_default;

pub fn all_menues(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Menu>>, Status> {
    let aux_limit = get_limit_or_default(limit, 10);
    menu_repository::show_menues(aux_limit, &connection)
        .map(|menu| Json(menu))
        .map_err(|error| error_status(error))
}

pub fn create_menu(new_menu: MenuDTO, connection: DbConn) ->  Result<status::Created<Json<Menu>>, Status> {
    menu_repository::create_menu(new_menu, &connection)
    .map(|menu| menu_created(menu))
    .map_err(|error| error_status(error))
}

pub fn get_menu_by_id(id: i32, connection: DbConn) -> Result<Json<Menu>, Status> {
    menu_repository::get_menu_by_id(id, &connection)
        .map(|menu| Json(menu))
        .map_err(|error| error_status(error))
}

pub fn update_menu_by_id(id: i32, menu_updated: Menu, connection: DbConn) -> Result<Json<Menu>, Status> {
    menu_repository::update_menu_by_id(id, menu_updated, &connection)
        .map(|menu| Json(menu))
        .map_err(|error| error_status(error))
}

pub fn delete_menu_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    menu_repository::delete_menu_by_id(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn menu_created(ing: Menu) -> status::Created<Json<Menu>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/menu/{id}", host = host(), port = port(), id = ing.id_menu).to_string(),
        Some(Json(ing)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}