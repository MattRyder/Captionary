use diesel::pg::PgConnection;
use diesel::connection::Connection;
use std::{thread, time};

use models::game::Game;
use models::round::Round;

pub struct Runner {
    db_connection: PgConnection
}

impl Runner {

    pub fn new(db_url: &String) -> Runner {
        let connection = PgConnection::establish(db_url)
        .expect(&format!("Runner: error connecting to {}", db_url));

        Runner { db_connection: connection }
    }

    pub fn start(&self) {
        let room_id = 1;
        let game = Game::create(&self.db_connection, room_id).unwrap();

        println!("Starting Game:\n{:?}", game);

        let round = Round::create(&self.db_connection, game.id).unwrap();
        println!("[Game #{}] Starting Round #{}", game.id, round.id);
        println!("[Game #{}] URL: {}", game.id, round.image_url);

        println!("TODO: Broadcast(RoundStart, game.id, round.id)");

        let ten_sec = time::Duration::from_millis(10 * 1000);
        thread::sleep(ten_sec);

        println!("[Game #{}] Submission finished. Setting flag on record.", game.id);
        round.set_submission_closed(&self.db_connection);

        println!("TODO: Broadcast(RoundVote, game.id, round.id)");
        thread::sleep(ten_sec);

        println!("[Game #{}] Round #{} over. Setting finished_at on record.", game.id, round.id);
        round.set_finished(&self.db_connection);



    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use std::env;

    #[test]
    fn should_create_new_instance() {
        dotenv::dotenv().ok();

        let db_url = env::var("DATABASE_URL").unwrap();
        let runner = Runner::new(&db_url);

        runner.start();
    }
}