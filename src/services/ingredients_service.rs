use crate::models::ingredient::Ingredient;

pub fn get_ingredients() -> Ingredient {
    let ing = Ingredient {
        name: "huevos".to_string(),
        price: 10.0,
        category: "lacteos".to_string(),
        brand: "San Juan".to_string(),
        description: "huevos de gallina".to_string(),
    };

    ing
}