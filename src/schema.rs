// @generated automatically by Diesel CLI.

diesel::table! {
    category (id_category) {
        id_category -> Int4,
        category_name -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    day (id_day) {
        id_day -> Int4,
        day_name -> Varchar,
    }
}

diesel::table! {
    food_plan (id_food_plan) {
        id_food_plan -> Int4,
        food_plan_name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    food_plan_day (id_plan_day) {
        id_plan_day -> Int4,
        id_food_plan -> Int4,
        id_day -> Int4,
        id_menu -> Int4,
    }
}

diesel::table! {
    food_plan_recipe (id_food_plan_recipe) {
        id_food_plan_recipe -> Int4,
        id_recipe -> Int4,
        id_food_plan -> Int4,
    }
}

diesel::table! {
    ingredient (id_ingredient) {
        id_ingredient -> Int4,
        ingredient_name -> Varchar,
    }
}

diesel::table! {
    menu (id_menu) {
        id_menu -> Int4,
        menu_name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    recipe (id_recipe) {
        id_recipe -> Int4,
        recipe_name -> Varchar,
        id_category -> Int4,
    }
}

diesel::table! {
    recipe_ingredient (id_recipe_ingredient) {
        id_recipe_ingredient -> Int4,
        id_ingrediente -> Int4,
        id_recipe -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    category,
    day,
    food_plan,
    food_plan_day,
    food_plan_recipe,
    ingredient,
    menu,
    posts,
    recipe,
    recipe_ingredient,
);
