use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::services::days_service;
use crate::sample::models::days::{Day, DayDTO};

#[get("/")]
pub fn all_days(connection: DbConn) -> Result<Json<Vec<Day>>, Status> {
    days_service::all_days(connection)
}

#[post("/", format ="application/json", data = "<new_day>")]
pub fn create_day(new_day: Json<DayDTO>, connection: DbConn) ->  Result<status::Created<Json<Day>>, Status> {
    days_service::create_day(new_day.into_inner(), connection)

}

#[get("/<id>")]
pub fn get_day_by_id(id: i32, connection: DbConn) -> Result<Json<Day>, Status> {
    days_service::get_day_by_id(id, connection)
}

#[put("/<id>", format = "application/json", data = "<day_updated>")]
pub fn update_day_by_id(id: i32, day_updated: Json<Day>, connection: DbConn) -> Result<Json<Day>, Status> {
    days_service::update_day_by_id(id, day_updated.into_inner(), connection)
}

#[delete("/<id>")]
pub fn delete_day_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    days_service::delete_day_by_id(id, connection)   
}
