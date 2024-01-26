-- Your SQL goes here
CREATE TABLE IF NOT EXISTS posts (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       body TEXT NOT NULL,
                       published BOOLEAN NOT NULL DEFAULT 'f'
);

-- ************************************************************
CREATE TABLE IF NOT EXISTS public.ingredient
(
    id_ingredient serial NOT NULL,
    ingredient_name VARCHAR NOT NULL DEFAULT 'unknown',
    CONSTRAINT ingredient_pk PRIMARY KEY (id_ingredient)
);

ALTER TABLE IF EXISTS public.ingredient
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.day
(
    id_day serial NOT NULL,
    day_name VARCHAR NOT NULL,
    PRIMARY KEY (id_day)
);

ALTER TABLE IF EXISTS public.day
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.menu
(
    id_menu serial NOT NULL,
    menu_name VARCHAR NOT NULL,
    PRIMARY KEY (id_menu)
);

ALTER TABLE IF EXISTS public.menu
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.category
(
    id_category serial NOT NULL,
    category_name VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 't',
    PRIMARY KEY (id_category)
);

ALTER TABLE IF EXISTS public.category
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.food_plan_day
(
    id_food_plan_day serial NOT NULL,
    id_food_plan integer NOT NULL,
    id_day integer NOT NULL,
    id_menu integer NOT NULL,
    PRIMARY KEY (id_food_plan_day)
);

ALTER TABLE IF EXISTS public.food_plan_day
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.food_plan
(
    id_food_plan serial NOT NULL,
    food_plan_name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    PRIMARY KEY (id_food_plan)
);


ALTER TABLE IF EXISTS public.food_plan
    OWNER to postgres;

-- ************************************************************

CREATE TABLE IF NOT EXISTS public.food_plan_recipe
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

CREATE TABLE IF NOT EXISTS public.recipe_ingredient
(
    id_recipe_ingredient serial NOT NULL,
    id_ingredient integer NOT NULL CONSTRAINT rec_ing_ing_fk REFERENCES ingredient(id_ingredient),
    id_recipe integer NOT NULL,
    PRIMARY KEY (id_recipe_ingredient),
	CONSTRAINT recipeingredient_recipe FOREIGN KEY (id_recipe)
        REFERENCES public.recipe (id_recipe) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
	CONSTRAINT recipeingredient_ingredient FOREIGN KEY (id_ingredient)
        REFERENCES public.ingredient (id_ingredient) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

ALTER TABLE IF EXISTS public.recipe_ingredient
    OWNER to postgres;

 -- ************************************************************

INSERT INTO day (day_name) VALUES ('Lunes'), ('Martes'), ('Miercoles'), ('Jueves'), ('Viernes'), ('Sabado'), ('Domingo');

CREATE OR REPLACE FUNCTION delete_empty_tables()
RETURNS VOID AS $$
DECLARE
  table_name VARCHAR;
  is_empty BOOLEAN;
BEGIN
  FOR table_name IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public' AND tablename <> '__diesel_schema_migrations') LOOP
    EXECUTE 'SELECT COUNT(*) = 0 FROM ' || table_name INTO is_empty;

    IF is_empty THEN
      EXECUTE 'DROP TABLE IF EXISTS ' || table_name;
    END IF;
	
  END LOOP;
END;
$$ LANGUAGE plpgsql;
