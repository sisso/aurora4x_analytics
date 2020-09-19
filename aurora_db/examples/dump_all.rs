extern crate rusqlite;
extern crate time;

use aurora_db::*;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/home/sisso/games/aurora11/AuroraDB.db");
    if !path.is_file() {
        panic!("file {:?} not found", path.canonicalize());
    }

    let db = AuroraDb::new(&path);
    let data = db.fetch().unwrap();

    for game in data.games {
        println!("{}", serde_json::to_string_pretty(&game).unwrap());
    }

    println!("done");
}
