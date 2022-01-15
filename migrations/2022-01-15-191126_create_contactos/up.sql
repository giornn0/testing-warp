-- Your SQL goes here
CREATE TABLE contactos (
  id SERIAL PRIMARY KEY,
  telefono VARCHAR(50) NOT NULL,
  email VARCHAR(50) NOT NULL,
  detalles VARCHAR(255) NOT NULL,
  jardin_id SERIAL REFERENCES jardines(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);