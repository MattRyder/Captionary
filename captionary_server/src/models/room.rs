#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;
use names::{Generator, Name};

use models::user::User;
use schema::rooms;
use schema::rooms::dsl::rooms as rooms_repo;
use schema::rooms::dsl::*;

const ROOM_MAX_CAPACITY: i64 = 8;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "rooms"]
pub struct NewRoom<'a> {
    pub name: &'a String,
}

#[derive(QueryableByName, Debug)]
#[table_name = "rooms"]
struct AvailableRoom {
    name: String,
}

impl Room {
    pub fn list_all(conn: &PgConnection) -> Vec<Room> {
        rooms_repo
            .load::<Room>(conn)
            .expect("Failed to load Room list_all")
    }

    pub fn find(conn: &PgConnection, room_name: &String) -> Result<Room, Error> {
        rooms::table
            .filter(rooms::name.eq(room_name))
            .first::<Room>(conn)
    }

    pub fn create<'a>(conn: &PgConnection) -> Result<Room, Error> {
        let mut generator = Generator::with_naming(Name::Numbered);

        let mut room_name: String;

        loop {
            room_name = generator.next().unwrap();
            let present = Self::is_name_available(conn, &room_name);
            println!(
                "Room slug {}: {} present",
                &room_name,
                if present { "IS" } else { "NOT" }
            );

            if !present {
                break;
            }
        }

        let new_room = NewRoom {
            name: &room_name.to_string(),
        };

        diesel::insert_into(rooms::table)
            .values(&new_room)
            .get_result(conn)
    }

    pub fn can_be_joined(&self, conn: &PgConnection) -> bool {
        let user_count = self.get_user_count(conn);

        user_count >= 0 && user_count < ROOM_MAX_CAPACITY
    }

    pub fn get_user_count(&self, conn: &PgConnection) -> i64 {
        User::belonging_to(self)
            .count()
            .get_result(conn)
            .ok()
            .unwrap()
    }

    pub fn find_available_room(conn: &PgConnection) -> Room {
        let find_room_sql = "
            SELECT name
            FROM rooms
            WHERE id = (
                SELECT room_id
                FROM users
                GROUP BY room_id
                HAVING COUNT(room_id) < 5
                ORDER BY count(room_id) DESC
                LIMIT 1
            );";

        let mut chosen_room : Option<Room> = None;

        let query_result : Vec<AvailableRoom> = sql_query(find_room_sql).load(conn).unwrap();
        if let Some(available_room) = query_result.first() {
            if let Ok(room) = Room::find(conn, &available_room.name) {
                chosen_room = Some(room);
            }
        }

        match chosen_room {
            Some(room) => room,
            None => Room::create(conn).unwrap(),
        }
    }

    fn is_name_available(conn: &PgConnection, room_name: &String) -> bool {
        let count: i64 = rooms::table
            .filter(name.eq(room_name))
            .count()
            .get_result(conn)
            .expect("Failed to find Room name availability.");

        return count > 0;
    }
}
