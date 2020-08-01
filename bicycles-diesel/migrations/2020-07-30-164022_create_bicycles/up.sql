-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE bicycles (
    id uuid DEFAULT uuid_generate_v4(),
    owner_id uuid NOT NULL,
    color VARCHAR(10),
    created_at TIMESTAMP DEFAULT now(),
    PRIMARY KEY (id)
)