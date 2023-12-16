use crate::models::ingredient::Ingredient;

pub fn get_ingredients() -> Ingredient {
    let ing = Ingredient {
        ingredient_id: 1,
        name: "huevos".to_string()
    };

    ing
}