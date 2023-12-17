use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::category_repository;
use crate::sample::models::category::Category;
use crate::sample::models::category::CategoryDTO;
use crate::sample::exceptions::errors::error_status;

pub fn all_categories(connection: DbConn) -> Result<Json<Vec<Category>>, Status> {
    category_repository::show_categories(&connection)
        .map(|category| Json(category))
        .map_err(|error| error_status(error))
}

pub fn create_category(new_category: CategoryDTO, connection: DbConn) ->  Result<status::Created<Json<Category>>, Status> {
    category_repository::create_category(new_category, &connection)
    .map(|category| category_created(category))
    .map_err(|error| error_status(error))
}

pub fn get_category_by_id(id: i32, connection: DbConn) -> Result<Json<Category>, Status> {
    category_repository::get_category_by_id(id, &connection)
        .map(|category| Json(category))
        .map_err(|error| error_status(error))
}

pub fn update_category_by_id(id: i32, category_updated: Category, connection: DbConn) -> Result<Json<Category>, Status> {
    category_repository::update_category_by_id(id, category_updated, &connection)
        .map(|category| Json(category))
        .map_err(|error| error_status(error))
}

pub fn delete_category_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    category_repository::delete_category_by_id(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn category_created(ing: Category) -> status::Created<Json<Category>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/category/{id}", host = host(), port = port(), id = ing.id_category).to_string(),
        Some(Json(ing)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}