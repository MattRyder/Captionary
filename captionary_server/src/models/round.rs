use chrono::NaiveDateTime;
use chrono::Utc;
use diesel;
use diesel::SaveChangesDsl;
use diesel::pg::PgConnection;
use diesel::prelude::{Insertable, Queryable, RunQueryDsl};

use schema::rounds;
use util::flickr::Flickr;

#[derive(Identifiable, Queryable, Debug)]
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
    pub submission_closed_at: Option<NaiveDateTime>
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "rounds"]
struct RoundFinishedParams {
    pub id: i32,
    pub finished_at: Option<NaiveDateTime>
}


impl Round {
    pub fn create<'a>(conn: &PgConnection, game_id: i32) -> Option<Round> {
        let new_round = NewRound {
            game_id: game_id,
            image_url: &Flickr::get_image_url().unwrap(),
        };

        diesel::insert_into(rounds::table)
            .values(&new_round)
            .get_result(conn)
            .ok()
    }

    pub fn set_submission_closed(&self, conn: &PgConnection) -> bool {
        let round_params = SubmissionClosedParams {
            id: self.id,
            submission_closed_at: Some(Utc::now().naive_utc())
        };

        round_params.save_changes::<Round>(conn).is_ok()
    }

    pub fn set_finished(&self, conn: &PgConnection) -> bool {
        let round_params = RoundFinishedParams {
            id: self.id,
            finished_at: Some(Utc::now().naive_utc())
        };

        round_params.save_changes::<Round>(conn).is_ok()
    }
}
