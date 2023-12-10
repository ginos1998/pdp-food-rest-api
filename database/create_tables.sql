BEGIN;

-- ************************************************************
CREATE TABLE public.ingredient
(
    ingredient_id serial NOT NULL,
    name character varying(20) NOT NULL DEFAULT 'unknown',
    CONSTRAINT ingredient_pk PRIMARY KEY (ingredient_id)
);

ALTER TABLE IF EXISTS public.ingredient
    OWNER to postgres;

-- ************************************************************

COMMIT;