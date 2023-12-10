BEGIN;

    CREATE
DATABASE food
    WITH
    OWNER = postgres
    ENCODING = 'UTF8'
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;

COMMENT
ON DATABASE food
    IS 'pdp-food-rest-api';

COMMIT;