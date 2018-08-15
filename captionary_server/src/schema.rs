table! {
    captions (id) {
        id -> Int4,
        text -> Text,
        points -> Int4,
        published_at -> Timestamptz,
        user_id -> Int4,
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
    users (id) {
        id -> Int4,
        room_id -> Nullable<Int4>,
        username -> Varchar,
        token -> Text,
        ip_address -> Varchar,
        created_at -> Timestamptz,
    }
}

joinable!(captions -> users (user_id));
joinable!(users -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    captions,
    rooms,
    users,
);
