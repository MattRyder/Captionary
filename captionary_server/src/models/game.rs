use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::{Identifiable, Insertable, RunQueryDsl};
use schema::games;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Game {
    pub id: i32,
    pub room_id: i32,
    pub created_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "games"]
pub struct NewGame {
    room_id: i32,
}

impl Game {
    pub fn create<'a>(conn: &PgConnection, room_id: i32) -> Option<Game> {
        let new_game = NewGame { room_id: room_id };

        diesel::insert_into(games::table)
            .values(&new_game)
            .get_result::<Game>(conn)
            .ok()
    }
}
