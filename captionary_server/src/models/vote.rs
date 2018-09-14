#![allow(proc_macro_derive_resolution_fallback)]

use diesel::result::Error;
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::caption::Caption;

use schema::votes;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "votes"]
#[belongs_to(Caption)]
pub struct Vote {
    pub id: i32,
    pub user_id: i32,
    pub caption_id: i32,
    pub submitted_at: NaiveDateTime
}

#[derive(Insertable, Queryable)]
#[table_name = "votes"]
pub struct NewVote {
    pub user_id: i32,
    pub caption_id: i32
}

impl Vote {
    pub fn create(connection: &PgConnection, users_id: i32, captions_id: i32) -> Result<Vote, Error> {
        let new_vote = NewVote { user_id: users_id, caption_id: captions_id };

        diesel::insert_into(votes::table)
            .values(&new_vote)
            .get_result(connection)
    }
}