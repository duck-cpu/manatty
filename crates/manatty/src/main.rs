use engine::database::CardDatabase;
use engine::game::deck_loading::{
    DeckList, PlayerDeckList, load_and_hydrate_decks, resolve_deck_list,
};
use engine::types::ObjectId;
use engine::types::format::FormatConfig;
use engine::types::game_state::GameState;
use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    // initialize game state
    let mut state = GameState::new(FormatConfig::commander(), 2, 42);

    let card_path = card_data_path();
    println!("Current DB: {}\n", card_path.display());

    // initiate decklist objects
    let deck_list = DeckList {
        player: PlayerDeckList {
            main_deck: vec![String::from("Lightning Bolt")],
            commander: vec![String::from("Sakashima of a Thousand Faces")],
            ..Default::default()
        },

        opponent: PlayerDeckList {
            main_deck: vec![String::from("Lightning Bolt")],
            commander: vec![String::from("Sakashima of a Thousand Faces")],
            ..Default::default()
        },

        ..Default::default()
    };

    // initialize card database (currently test db from data/mtgjson/test_fixture.json)
    let db = match CardDatabase::from_mtgjson(&card_path) {
        Ok(db) => {
            // match db.get_face_by_name("Lightning Bolt") {
            //    Some(card) => {
            //        println!("Name: {}\n", card.name);
            //        println!("Mana Cost: {:?}\n", card.mana_cost);
            //    }
            //    None => {
            //        eprintln!("No card found.");
            //        process::exit(1);
            //    }
            // };
            db
        }
        Err(error) => {
            eprintln!("ERROR: {}, {} ", card_path.display(), error);
            process::exit(1);
        }
    };
    // initialize payload using the database, and decklist
    let payload = resolve_deck_list(&db, &deck_list);

    // load payload into game state
    load_and_hydrate_decks(&mut state, &payload, Some(&db));

    // load ObjectId from players library and prints info
    for id in state.players.iter() {
        let pobj = match id.library.get(0) {
            Some(obj) => obj,
            None => {
                eprintln!("No object for this owner.\n");
                continue;
            }
        };
        print_object_state(&state, &pobj);
    }
    // load ObjectId from command zone and prints info
    for id in state.command_zone.iter() {
        print_object_state(&state, id);
    }
}

// set path to card database as a CLI arg
fn card_data_path() -> PathBuf {
    let mut args = env::args_os();

    args.next();

    if let Some(arg) = args.next() {
        return PathBuf::from(arg);
    }

    match env::var_os("MANATTY_CARDS_PATH") {
        Some(value) => PathBuf::from(value),
        None => panic!("No path you fool...1"),
    }
}

// convenient debug function to print game object state
fn print_object_state(state: &GameState, pobj: &ObjectId) {
    if let Some(obj) = state.objects.get(pobj) {
        println!(
            "Name: {}\nOwner: {:?}\nZone: {:?}\n",
            obj.name, obj.owner, obj.zone
        );
    }
}
