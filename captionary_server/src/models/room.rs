use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::result::Error;
use names::{Generator, Name};

use schema::rooms;
use models::user::User;
use schema::rooms::dsl::rooms as rooms_repo;
use schema::rooms::dsl::*;

const ROOM_MAX_CAPACITY : i64 = 8;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug)]
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
    id: i32,
    name: String,
    created_at: NaiveDateTime,
}

impl Room {
    pub fn list_all(conn: &PgConnection) -> Vec<Room> {
        rooms_repo
            .load::<Room>(conn)
            .expect("Failed to load Room list_all")
    }

    pub fn find(conn: &PgConnection, room_id: i32) -> Result<Room, Error> {
        rooms::table.find(room_id).first::<Room>(conn)
    }

    pub fn create<'a>(conn: &PgConnection) -> Result<Room, Error> {
        let mut generator = Generator::with_naming(Name::Numbered);

        let room_name = generator.next().unwrap();
        // let mut room_name: String;
        // while {
            // room_name = generator.next().unwrap();
            // let present = Self::is_name_available(conn, &room_name);
        //     present == false
        // } {}

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
                .ok().unwrap()
    }

    pub fn find_available_room_id(conn: &PgConnection) -> i32 {
        let find_room_sql = "
            SELECT *
            FROM rooms
            WHERE id = (
                SELECT room_id
                FROM users
                GROUP BY room_id
                HAVING COUNT(room_id) < 5
                ORDER BY count(room_id) DESC
                LIMIT 1
            );";

        let res : Vec<AvailableRoom> = sql_query(find_room_sql).load(conn).unwrap();
        let first_room = res.first();

        match first_room {
            Some(room) => room.id,
            None => {
                match Self::create(conn) {
                    Ok(room) => room.id,
                    Err(_) => 0
                }
            }
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
