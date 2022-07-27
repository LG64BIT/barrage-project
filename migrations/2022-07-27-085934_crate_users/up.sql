-- Your SQL goes here
CREATE TABLE users (
    id varchar(36) DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    email varchar(255) NOT NULL UNIQUE,
    "password" varchar(255) NOT NULL,
    is_admin boolean NOT NULL DEFAULT 'false',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();