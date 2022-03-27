create table if not exists leg (
    id integer not null primary key,

    created_by integer not null,
    created_date timestamptz not null,

    description text null,
    distance double precision not null,
    origin text not null,
    destination text not null,
    additional_waypoints text null,

    foreign key (created_by) references pilot (id)
)
