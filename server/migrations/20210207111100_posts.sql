CREATE TABLE posts (
    id serial PRIMARY KEY,
    title varchar(255) NOT NULL,
    text text NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    created_by integer NOT NULL REFERENCES users (id)
);