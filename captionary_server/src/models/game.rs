#![allow(proc_macro_derive_resolution_fallback)]

use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::ExpressionMethods;
use diesel::pg::PgConnection;
use diesel::prelude::{QueryDsl, RunQueryDsl};
use diesel::result::Error;
use diesel::{BelongingToDsl, SaveChangesDsl};
use models::round::Round;
use models::room::Room;

use schema::games;

use std::env;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "games"]
#[belongs_to(Room)]
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

#[derive(AsChangeset, Identifiable)]
#[table_name = "games"]
pub struct GameFinishedParams {
    pub id: i32,
    pub finished_at: Option<NaiveDateTime>
}

impl Game {
    pub fn find(connection: &PgConnection, game_id: i32) -> Result<Game, Error> {
        games::table.find(game_id).first::<Game>(connection)
    }

    pub fn create<'a>(conn: &PgConnection, room_id: i32) -> Option<Game> {
        let new_game = NewGame { room_id: room_id };

        diesel::insert_into(games::table)
            .values(&new_game)
            .get_result::<Game>(conn)
            .ok()
    }

    pub fn get_last_round(&self, connection: &PgConnection) -> Option<Round> {
        use schema::rounds::dsl::id;

        Round::belonging_to(self)
            .order(id.desc())
            .first(connection)
            .ok()
    }

    pub fn get_round_count(&self, connection: &PgConnection) -> i64 {
        Round::belonging_to(self)
            .count()
            .get_result(connection)
            .ok()
            .unwrap()
    }

    pub fn can_start_round(&self, connection: &PgConnection) -> bool {
        let round_count = self.get_round_count(connection);

        let max_rounds = env::var("GAME_MAX_ROUNDS").unwrap().parse::<i64>().unwrap();
        round_count >= 0 && round_count < max_rounds
    }

    pub fn start_round(&self, connection: &PgConnection) -> Option<Round> {
        match self.can_start_round(&connection) {
            true => Round::create(&connection, self.id),
            false => None
        }
    }

    pub fn set_finished(&self, connection: &PgConnection) -> Result<Self, Error> {
        let params = GameFinishedParams {
            id: self.id,
            finished_at: Some(Utc::now().naive_utc()),
        };

        params.save_changes::<Game>(connection)
    }

}
