use engine::types::game_state::GameState;
use engine::types::format::FormatConfig;
use std::path::PathBuf;
use std::env;

fn main() {
    //GameState::new(FormatConfig::format, number of players, seed);
    let state = GameState::new(
        FormatConfig::commander(),
        2,
        42);

    println!("players: {}", state.players.len());
    println!("waiting for.. {:?}", state.waiting_for);

    let card_path = card_data_path();
    println!("card database: {}", card_path.display());
}

fn card_data_path() -> PathBuf {
    let mut args = env::args_os();
    
    args.next();

    if let Some(arg) = args.next() {
        return PathBuf::from(arg);
    }

    match env::var_os("MANATTY_CARDS_PATH") {
        Some(value) => PathBuf::from(value),
        None => panic!("no path you fool"),
    }
}
