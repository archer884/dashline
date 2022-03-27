create table if not exists pilot (
    id serial primary key,
    name text not null unique,
    created_date timestamptz not null
)
