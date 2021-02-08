CREATE TABLE users (
     id serial PRIMARY KEY,
     email varchar(255) NOT NULL,
     name varchar(255) NOT NULL,
     password varchar(255) NOT NULL,
     date timestamp with time zone NOT NULL DEFAULT now()
);