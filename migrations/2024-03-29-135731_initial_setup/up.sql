-- Your SQL goes here
create table todo (
    id serial primary key,
    title text not null,
    pos serial
)