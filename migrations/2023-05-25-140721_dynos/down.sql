-- Add down migration script here

DROP TABLE IF EXISTS "dynos";
DROP TRIGGER IF EXISTS "generate_data_url";
