#![allow(proc_macro_derive_resolution_fallback)]

use std::env;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::{QueryDsl, RunQueryDsl};
use diesel::result::Error;
use diesel::SaveChangesDsl;
use diesel::BelongingToDsl;

use flickr::Flickr;
use models::caption::Caption;
use models::game::Game;
use schema::rounds;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "rounds"]
#[belongs_to(Game)]
pub struct Round {
    pub id: i32,
    pub game_id: i32,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub submission_closed_at: Option<NaiveDateTime>,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "rounds"]
pub struct NewRound<'a> {
    game_id: i32,
    image_url: &'a String,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "rounds"]
struct SubmissionClosedParams {
    pub id: i32,
    pub submission_closed_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "rounds"]
struct RoundFinishedParams {
    pub id: i32,
    pub finished_at: Option<NaiveDateTime>,
}

impl Round {
    pub fn get_captions(&self, connection: &PgConnection) -> Result<Vec<Caption>, Error> {
        Caption::belonging_to(self).load::<Caption>(connection)
    }

    pub fn get_winning_caption(&self, connection: &PgConnection) -> Option<Caption> {
        if let Ok(captions) = self.get_captions(connection) {
            captions.into_iter().max_by_key(|c| c.points)
        } else {
            None
        }
    }

    pub fn find(connection: &PgConnection, round_id: i32) -> Result<Round, Error> {
        rounds::table.find(round_id).first::<Round>(connection)
    }

    pub fn create<'a>(conn: &PgConnection, game_id: i32) -> Option<Round> {
        let flickr_api_key = env::var("FLICKR_KEY").expect("Please set env_var FLICKR_KEY");

        let new_round = NewRound {
            game_id: game_id,
            image_url: &Flickr::get_image_url(&flickr_api_key).unwrap(),
        };

        diesel::insert_into(rounds::table)
            .values(&new_round)
            .get_result(conn)
            .ok()
    }

    pub fn set_submission_closed(&self, conn: &PgConnection) -> bool {
        let round_params = SubmissionClosedParams {
            id: self.id,
            submission_closed_at: Some(Utc::now().naive_utc()),
        };

        round_params.save_changes::<Round>(conn).is_ok()
    }

    pub fn set_finished(&self, conn: &PgConnection) -> bool {
        let round_params = RoundFinishedParams {
            id: self.id,
            finished_at: Some(Utc::now().naive_utc()),
        };

        round_params.save_changes::<Round>(conn).is_ok()
    }
}
