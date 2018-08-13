use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_connection_pool(db_url: String) -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::new(manager).expect("Failed to initialize connection pool.")
}

pub struct DatabaseConnection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DatabaseConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DatabaseConnection, ()> {
        let pool = request.guard::<State<ConnectionPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DatabaseConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DatabaseConnection {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
