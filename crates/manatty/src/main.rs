use engine::types::game_state::GameState;
use engine::types::format::FormatConfig;
use engine::database::CardDatabase;
use std::path::PathBuf;
use std::env;
use std::process;

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

    match CardDatabase::from_mtgjson(&card_path) {
        Ok(db) => {
            match db.get_face_by_name("Lightning Bolt") {
                Some(card) => {
                    println!("Name: {}", card.name);
                    println!("Mana Cost: {:?}", card.mana_cost);
                },
                None => {
                    eprintln!("No card found.");
                    process::exit(1);
                },
            }
        },
        Err(error) => {
            eprintln!("ERROR: {}, {} ", card_path.display(), error);
            process::exit(1);
        },
    }
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
