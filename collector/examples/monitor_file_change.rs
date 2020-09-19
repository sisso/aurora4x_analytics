use collector::{collect, CollectorCfg};
use notify::{watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let path = "/home/sisso/games/aurora11/AuroraDBSaveBackup.db";

    println!("monitoring {}", path);

    collect(CollectorCfg {
        db_path: path.to_string(),
        output_path: "/tmp/01.log".to_string(),
    })
    .unwrap();
}
