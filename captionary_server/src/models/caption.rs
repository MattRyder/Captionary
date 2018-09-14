#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use diesel;
use diesel::BelongingToDsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use models::round::Round;
use models::vote::Vote;

use schema::captions;
use schema::captions::dsl::captions as captions_repo;
use schema::captions::dsl::id;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, QueryableByName, Debug)]
#[table_name = "captions"]
#[belongs_to(Round)]
pub struct Caption {
    pub id: i32,
    pub text: String,
    pub points: i32,
    pub published_at: NaiveDateTime,
    pub user_id: i32,
    pub round_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "captions"]
pub struct NewCaption<'a> {
    pub round_id: i32,
    pub user_id: i32,
    pub text: &'a String,
}

impl Caption {
    pub fn list_all(conn: &PgConnection) -> Vec<Caption> {
        captions_repo
            .load::<Caption>(conn)
            .expect("Failed to load Caption list_all")
    }

    pub fn find(connection: &PgConnection, caption_id: i32) -> Result<Caption, Error> {
        captions::table
            .find(caption_id)
            .first::<Caption>(connection)
    }

    pub fn create<'a>(
        conn: &PgConnection,
        round_id: i32,
        user_id: i32,
        text: &String,
    ) -> Caption {
        let new_caption = NewCaption { round_id, user_id, text };

        diesel::insert_into(captions::table)
            .values(&new_caption)
            .get_result(conn)
            .expect("Failed to create Caption.")
    }

    pub fn destroy(&self, conn: &PgConnection) -> bool {
        diesel::delete(captions::table.filter(id.eq(self.id)))
            .execute(conn)
            .is_ok()
    }

    pub fn get_votes(&self, connection: &PgConnection) -> Result<Vec<Vote>, Error> {
        Vote::belonging_to(self).load::<Vote>(connection)
    }
}
