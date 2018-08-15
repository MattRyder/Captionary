table! {
    captions (id) {
        id -> Int4,
        text -> Text,
        points -> Int4,
        published_at -> Timestamptz,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        token -> Varchar,
        ip_address -> Varchar,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    captions,
    rooms,
    sessions,
);
