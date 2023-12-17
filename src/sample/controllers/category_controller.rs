use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::services::category_service;
use crate::sample::models::category::{Category, CategoryDTO};

#[get("/?<limit>")]
pub fn all_categorys(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Category>>, Status> {
    category_service::all_categories(limit, connection)
}

#[post("/", format ="application/json", data = "<new_category>")]
pub fn create_category(new_category: Json<CategoryDTO>, connection: DbConn) ->  Result<status::Created<Json<Category>>, Status> {
    category_service::create_category(new_category.into_inner(), connection)

}

#[get("/<id>")]
pub fn get_category_by_id(id: i32, connection: DbConn) -> Result<Json<Category>, Status> {
    category_service::get_category_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<category_updated>")]
pub fn update_category_by_id(id: i32, category_updated: Json<Category>, connection: DbConn) -> Result<Json<Category>, Status> {
    category_service::update_category_by_id(id, category_updated.into_inner(), connection)
}

#[delete("/<id>")]
pub fn delete_category_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    category_service::delete_category_by_id(id, connection)   
}
