use amqp::{Client, Message};
use chrono::Duration;
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::{Identifiable, Insertable, QueryDsl, RunQueryDsl};
use diesel::result::Error;
use diesel::BelongingToDsl;
use models::round::Round;
use schema::games;

use std::env;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "games"]
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

    pub fn start_round(&self, connection: &PgConnection, amqp_client: &Client) {
        if self.can_start_round(&connection) {
            let round = Round::create(&connection, self.id).unwrap();

            let message = Message::SubmissionClosed(round.id);
            let event_delay = Duration::milliseconds(30 * 1000);
            amqp_client.publish(message, event_delay);

            let message2 = Message::RoundFinished(round.id);
            let event_delay2 = Duration::milliseconds(60 * 1000);
            amqp_client.publish(message2, event_delay2);
        }
    }
}
