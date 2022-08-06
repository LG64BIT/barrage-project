-- Your SQL goes here
CREATE TABLE products (
    id varchar(36) DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    price INT NOT NULL
);