CREATE TABLE IF NOT EXISTS movies (
    id bigserial PRIMARY KEY,
    title varchar(255) unique,
    director text
);

create table if not exists users (
    id bigserial primary key,
    first_name varchar(255) not null ,
    last_name varchar(255) not null ,
    age smallint default 0
);