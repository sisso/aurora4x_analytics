use collector::*;
use notify::{watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let output_path = "./01.log";
    let paths = vec![
        "/home/sisso/home/shared/aurora11/AuroraDB.db",
        "/home/sisso/home/shared/aurora11/AuroraDBPreviousSaveBackup.db",
        "/home/sisso/home/shared/aurora11/AuroraDBSaveBackup.db",
    ];
    // let path = "/home/sisso/games/aurora11/AuroraDB.db";

    for path in paths {
        println!("processing {}", path);
        append_output(path, output_path).unwrap();
    }
}
