use chrono::NaiveDateTime;
use diesel;
use diesel::sql_query;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use names::{Generator, Name};

use schema::rooms;
use schema::rooms::dsl::rooms as rooms_repo;
use schema::rooms::dsl::*;

#[derive(Serialize, Deserialize, Queryable, Debug)]
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

impl Room {
    pub fn list_all(conn: &PgConnection) -> Vec<Room> {
        rooms_repo
            .load::<Room>(conn)
            .expect("Failed to load Room list_all")
    }

    pub fn find_available_room(conn: &PgConnection) {
        
        // let find_room_sql = "
        //     SELECT * 
        //     FROM rooms
        //     WHERE id IN (
        //         SELECT room_id
        //         FROM users
        //         GROUP BY room_id
        //         HAVING COUNT(room_id) < 5
        //     );";

        

        // let res = sql_query(find_room_sql).load::<Room>(conn);
    }

    pub fn find(conn: &PgConnection, room_id: i32) -> Result<Room, Error> {
        rooms::table.find(room_id).first::<Room>(conn)
    }

    pub fn create<'a>(conn: &PgConnection) -> Room {
        let mut generator = Generator::with_naming(Name::Numbered);
        
        let mut room_name : String;
        while {
            room_name = generator.next().unwrap();
            let present = Self::is_name_available(conn, &room_name);
            present == false
        } {}

        let new_room = NewRoom {
            name: &room_name.to_string(),
        };

        diesel::insert_into(rooms::table)
            .values(&new_room)
            .get_result(conn)
            .expect("Failed to create Room.")
    }

    fn is_name_available(conn: &PgConnection, room_name: &String) -> bool {
        let count : i64 = rooms::table
            .filter(name.eq(room_name))
            .count()
            .get_result(conn)
            .expect("Failed to find Room name availability.");

        return count > 0;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn list_all() {
        unimplemented!();
    }
}
