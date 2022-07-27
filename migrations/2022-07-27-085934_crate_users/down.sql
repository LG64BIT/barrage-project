-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON users;
DROP FUNCTION trigger_set_timestamp;
DROP TABLE users;