-- Your SQL goes here
CREATE TABLE if not exists posts (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       body TEXT NOT NULL,
                       published BOOLEAN NOT NULL DEFAULT 'f'
);

-- ************************************************************
CREATE TABLE public.ingredient
(
    id_ingredient serial NOT NULL,
    ingredient_name VARCHAR NOT NULL DEFAULT 'unknown',
    CONSTRAINT ingredient_pk PRIMARY KEY (id_ingredient)
);

ALTER TABLE IF EXISTS public.ingredient
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.day
(
    id_day serial NOT NULL,
    day_name VARCHAR NOT NULL,
    PRIMARY KEY (id_day)
);

ALTER TABLE IF EXISTS public.day
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.menu
(
    id_menu serial NOT NULL,
    menu_name VARCHAR NOT NULL,
    PRIMARY KEY (id_menu)
);

ALTER TABLE IF EXISTS public.menu
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.category
(
    id_category serial NOT NULL,
    category_name VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 't',
    PRIMARY KEY (id_category)
);

ALTER TABLE IF EXISTS public.category
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.food_plan_day
(
    id_plan_day serial NOT NULL,
    id_food_plan integer NOT NULL,
    id_day integer NOT NULL,
    id_menu integer NOT NULL,
    PRIMARY KEY (id_plan_day)
);

ALTER TABLE IF EXISTS public.food_plan_day
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.food_plan
(
    id_food_plan serial NOT NULL,
    food_plan_name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    PRIMARY KEY (id_food_plan)
);


ALTER TABLE IF EXISTS public.food_plan
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.food_plan_recipe
(
    id_food_plan_recipe serial NOT NULL,
    id_recipe integer NOT NULL,
    id_food_plan integer NOT NULL,
    PRIMARY KEY (id_food_plan_recipe)
);

ALTER TABLE IF EXISTS public.food_plan_recipe
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.recipe
(
    id_recipe serial NOT NULL,
    recipe_name VARCHAR NOT NULL,
    id_category integer NOT NULL,
    CONSTRAINT recipe_pkey PRIMARY KEY (id_recipe)
);

ALTER TABLE IF EXISTS public.recipe
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.recipe_ingredient
(
    id_recipe_ingredient serial NOT NULL,
    id_ingrediente integer NOT NULL,
    id_recipe integer NOT NULL,
    PRIMARY KEY (id_recipe_ingredient)
);

ALTER TABLE IF EXISTS public.recipe_ingredient
    OWNER to postgres;

 -- ************************************************************
