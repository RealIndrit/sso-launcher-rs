use crate::api::GameStatus;
use anyhow::Error;

pub fn status_game(game_status: GameStatus) -> Result<(), Error> {
    println!("{:?}", game_status);
    Ok(())
}
