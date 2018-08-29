#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use schema::captions;
use schema::captions::dsl::captions as captions_repo;
use schema::captions::dsl::*;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Caption {
    pub id: i32,
    pub text: String,
    pub points: i32,
    pub published_at: NaiveDateTime,
    pub user_id: i32,
    pub round_id: i32
}

#[derive(Insertable, Debug)]
#[table_name = "captions"]
pub struct NewCaption<'a> {
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

    pub fn create<'a>(conn: &PgConnection, caption_text: &String) -> Caption {
        let new_caption = NewCaption { text: caption_text };

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
}
