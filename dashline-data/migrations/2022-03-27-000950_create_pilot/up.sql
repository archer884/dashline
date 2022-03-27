create table if not exists pilot (
    id integer not null primary key,
    name text not null unique,
    created_date timestamptz not null
)
