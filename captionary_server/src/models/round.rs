use chrono::{NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::{Insertable, Queryable, RunQueryDsl};

use schema::rounds;

#[derive(Queryable, Debug)]
pub struct Round {
    pub id: i32,
    pub game_id: i32,
    pub image_url: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "rounds"]
pub struct NewRound<'a> {
    game_id: i32,
    image_url: &'a String
}

impl Round {

    pub fn create<'a>(conn: &PgConnection, game_id: i32) -> Option<Round> {
        let new_round = NewRound { 
            game_id: game_id,
            image_url: &("IMAGE_URL_HERE".into()) };

        diesel::insert_into(rounds::table)
            .values(&new_round)
            .get_result(conn)
            .ok()
    }

}