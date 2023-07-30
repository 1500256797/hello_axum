-- Add migration script here
CREATE TABLE model_order_relation(
  id serial PRIMARY KEY,
  model_id varchar(11) NOT NULL,
  order_hash varchar(255) NOT NULL,
  status int DEFAULT 1,
  created_at timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);
