extern crate rusqlite;
extern crate time;
extern crate error_chain;

use std::path::PathBuf;
use aurora_db::*;

mod aurora_db;

fn main() {
    let path = PathBuf::from("/home/sisso/games/aurora11/AuroraDB.db");
    if !path.is_file() {
        panic!("file {:?} not found", path.canonicalize());
    }

    let db = AuroraDb::new(path);
    println!("{:?}", db.fetch());


    println!("done");
}
