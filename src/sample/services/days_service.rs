use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample::repositories::days_repository;
use crate::sample::models::days::Day;
use crate::sample::models::days::DayDTO;
use crate::sample::exceptions::errors::error_status;
use crate::sample::utils::common_functions::get_limit_or_default;

pub fn all_days(limit: Option<i64>, connection: DbConn) -> Result<Json<Vec<Day>>, Status> {
    let aux_limit = get_limit_or_default(limit, 5);
    days_repository::show_days(aux_limit, &connection)
        .map(|day| Json(day))
        .map_err(|error| error_status(error))
}

pub fn create_day(new_day: DayDTO, connection: DbConn) ->  Result<status::Created<Json<Day>>, Status> {
    days_repository::create_day(new_day, &connection)
    .map(|day| day_created(day))
    .map_err(|error| error_status(error))
}

pub fn get_day_by_id(id: i32, connection: DbConn) -> Result<Json<Day>, Status> {
    days_repository::get_day_by_id(id, &connection)
        .map(|day| Json(day))
        .map_err(|error| error_status(error))
}

pub fn update_day_by_id(id: i32, day_updated: Day, connection: DbConn) -> Result<Json<Day>, Status> {
    days_repository::update_day_by_id(id, day_updated, &connection)
        .map(|day| Json(day))
        .map_err(|error| error_status(error))
}

pub fn delete_day_by_id(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    days_repository::delete_day_by_id(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn day_created(ing: Day) -> status::Created<Json<Day>> {
    println!("here final");
    status::Created(
        format!("{host}:{port}/day/{id}", host = host(), port = port(), id = ing.id_day).to_string(),
        Some(Json(ing)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}