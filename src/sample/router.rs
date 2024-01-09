use rocket;

use crate::connection;
use crate::sample;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/posts",
               routes![
                    sample::controllers::posts_controller::all_posts,
                    sample::controllers::posts_controller::create_post,
                    sample::controllers::posts_controller::get_post,
                    sample::controllers::posts_controller::update_post,
                    sample::controllers::posts_controller::delete_post
                    ],
        )
        .mount("/ingredients", 
               routes![
                    sample::controllers::ingredient_controller::all_ingredients,
                    sample::controllers::ingredient_controller::create_ingredient,
                    sample::controllers::ingredient_controller::get_ingredient_by_id,
                    sample::controllers::ingredient_controller::update_ingredient_by_id,
                    sample::controllers::ingredient_controller::delete_ingredient_by_id
                    ], )
        .mount("/category",
                routes![
                      sample::controllers::category_controller::all_categorys,
                      sample::controllers::category_controller::create_category,
                      sample::controllers::category_controller::get_category_by_id,
                      sample::controllers::category_controller::update_category_by_id,
                      sample::controllers::category_controller::delete_category_by_id
                      ], )
        .mount("/recipe",
                routes![
                      sample::controllers::recipe_controller::all_recipes,
                      sample::controllers::recipe_controller::create_recipe,
                      sample::controllers::recipe_controller::get_recipe_by_id,
                      sample::controllers::recipe_controller::update_recipe_by_id,
                      sample::controllers::recipe_controller::delete_recipe_by_id,
                      sample::controllers::recipe_controller::create_custom_recipe
                      ], )
        .mount("/day",
                routes![
                      sample::controllers::days_controller::all_days,
                      sample::controllers::days_controller::create_day,
                      sample::controllers::days_controller::get_day_by_id,
                      sample::controllers::days_controller::update_day_by_id,
                      sample::controllers::days_controller::delete_day_by_id
                      ], )
        .mount("/menu",
                routes![
                      sample::controllers::menu_controller::all_menues,
                      sample::controllers::menu_controller::create_menu,
                      sample::controllers::menu_controller::get_menu_by_id,
                      sample::controllers::menu_controller::update_menu_by_id,
                      sample::controllers::menu_controller::delete_menu_by_id
                      ], )
        .mount("/food-plan",
                routes![
                      sample::controllers::food_plan_controller::all_plans,
                      sample::controllers::food_plan_controller::create_plan,
                      sample::controllers::food_plan_controller::get_plan_by_id,
                      sample::controllers::food_plan_controller::update_plan_by_id,
                      sample::controllers::food_plan_controller::delete_plan_by_id,
                      sample::controllers::food_plan_controller::create_custom_plan
                      ],)
        .launch();
}