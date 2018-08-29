#![allow(proc_macro_derive_resolution_fallback)]

table! {
    captions (id) {
        id -> Int4,
        text -> Text,
        points -> Int4,
        published_at -> Timestamptz,
        user_id -> Int4,
        round_id -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        room_id -> Int4,
        created_at -> Timestamptz,
        finished_at -> Nullable<Timestamptz>,
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
    rounds (id) {
        id -> Int4,
        game_id -> Int4,
        image_url -> Text,
        created_at -> Timestamptz,
        submission_closed_at -> Nullable<Timestamptz>,
        finished_at -> Nullable<Timestamptz>,
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

joinable!(captions -> rounds (round_id));
joinable!(captions -> users (user_id));
joinable!(games -> rooms (room_id));
joinable!(rounds -> games (game_id));
joinable!(users -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    captions,
    games,
    rooms,
    rounds,
    users,
);
