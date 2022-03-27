table! {
    flight (id) {
        id -> Int4,
        pilot_id -> Int4,
        leg_id -> Int4,
        created_date -> Timestamp,
        completed_date -> Timestamptz,
        remarks -> Nullable<Text>,
        aircraft -> Text,
        duration -> Nullable<Text>,
    }
}

table! {
    leg (id) {
        id -> Int4,
        created_by -> Int4,
        created_date -> Timestamptz,
        description -> Nullable<Text>,
        distance -> Float8,
        origin -> Text,
        destination -> Text,
        additional_waypoints -> Nullable<Text>,
    }
}

table! {
    pilot (id) {
        id -> Int4,
        name -> Text,
        created_date -> Timestamptz,
    }
}

joinable!(flight -> leg (leg_id));
joinable!(flight -> pilot (pilot_id));
joinable!(leg -> pilot (created_by));

allow_tables_to_appear_in_same_query!(flight, leg, pilot,);
