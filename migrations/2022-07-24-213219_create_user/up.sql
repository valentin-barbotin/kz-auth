-- -- Your SQL goes here

-- create table users (
--   id serial primary key,
--   name varchar(255) not null,
--   email varchar(255) not null,
--   password varchar(255) not null,
--   created_at timestamp not null default now(),
--   updated_at timestamp not null default now(),
--   CONSTRAINT users_email_unique UNIQUE (email),
--   CONSTRAINT users_name_unique UNIQUE (name)
-- );

create table auth.users
(
    id         serial primary key,
    name       varchar(255)            not null,
    email      varchar(255)            not null,
    password   varchar(255)            not null,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null,
    CONSTRAINT users_email_unique UNIQUE (email),
    CONSTRAINT users_name_unique UNIQUE (name)
);

create index users_email_index
    on auth.users (email);

create index users_name_email_index
    on auth.users (name, email);

