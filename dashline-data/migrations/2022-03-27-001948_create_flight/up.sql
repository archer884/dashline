create table if not exists flight (
    id integer not null primary key,
    pilot_id integer not null,
    leg_id integer not null,

    created_date timestamp not null,
    completed_date timestamptz not null,

    remarks text null,
    aircraft text not null,
    -- This is meant to be an actual duration type, but the database has no
    -- way of representing that, so just be careful.
    duration text null,

    foreign key (pilot_id) references pilot (id),
    foreign key (leg_id) references leg (id)
)
