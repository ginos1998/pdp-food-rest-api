use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::services::menu_service;
use crate::sample::models::menu::{Menu, MenuDTO};

#[get("/?<limit>")]
pub fn all_menues(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Menu>>, Status> {
    menu_service::all_menues(limit, connection)
}

#[post("/", format ="application/json", data = "<new_menu>")]
pub fn create_menu(new_menu: Json<MenuDTO>, connection: DbConn) ->  Result<status::Created<Json<Menu>>, Status> {
    menu_service::create_menu(new_menu.into_inner(), connection)

}

#[get("/<id>")]
pub fn get_menu_by_id(id: i32, connection: DbConn) -> Result<Json<Menu>, Status> {
    menu_service::get_menu_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<menu_updated>")]
pub fn update_menu_by_id(id: i32, menu_updated: Json<Menu>, connection: DbConn) -> Result<Json<Menu>, Status> {
    menu_service::update_menu_by_id(id, menu_updated.into_inner(), connection)
}

#[delete("/<id>")]
pub fn delete_menu_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    menu_service::delete_menu_by_id(id, connection)   
}
