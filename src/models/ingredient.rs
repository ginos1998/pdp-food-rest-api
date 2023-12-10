use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub name: String,
    pub price: f32,
    pub category: String,
    pub brand: String,
    pub description: String,
}
