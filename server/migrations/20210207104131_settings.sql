CREATE TABLE settings (
     id serial PRIMARY KEY,
     key varchar(255) NOT NULL,
     value text NOT NULL,
     created_at timestamp with time zone NOT NULL DEFAULT now()
);