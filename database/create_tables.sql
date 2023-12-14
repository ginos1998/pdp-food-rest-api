BEGIN;

-- ************************************************************
CREATE TABLE public.ingredient
(
    id_ingredient serial NOT NULL,
    name character varying(20) NOT NULL DEFAULT 'unknown',
    CONSTRAINT ingredient_pk PRIMARY KEY (id_ingredient)
);

ALTER TABLE IF EXISTS public.ingredient
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.day
(
    id_day serial NOT NULL,
    day character varying(20)[] NOT NULL,
    PRIMARY KEY (id_day)
);

ALTER TABLE IF EXISTS public.day
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.menu
(
    id_menu serial NOT NULL,
    name character varying(20)[] NOT NULL,
    PRIMARY KEY (id_menu)
);

ALTER TABLE IF EXISTS public.menu
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.category
(
    id_category serial NOT NULL,
    name character varying(20)[],
    active character varying(20)[],
    PRIMARY KEY (id_category)
)

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
    name character varying(20)[],
    description character varying(60)[],
    PRIMARY KEY (id_food_plan)
)


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

CREATE TABLE IF NOT EXISTS public.receipe
(
    id_receipe serial NOT NULL,
    name character varying(20)[] COLLATE pg_catalog."default",
    id_category integer NOT NULL,
    CONSTRAINT receipe_pkey PRIMARY KEY (id_receipe)
)

ALTER TABLE IF EXISTS public.receipe
    OWNER to postgres;

-- ************************************************************

CREATE TABLE public.receipe_ingredient
(
    id_receipe_ingredient serial NOT NULL,
    id_ingrediente integer NOT NULL,
    id_receipe integer NOT NULL,
    PRIMARY KEY (id_receipe_ingredient)
);

ALTER TABLE IF EXISTS public.receipe_ingredient
    OWNER to postgres;

 -- ************************************************************
    
COMMIT;
