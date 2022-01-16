-- Your SQL goes here
-- Your SQL goes here
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON jardines
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON contactos
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();