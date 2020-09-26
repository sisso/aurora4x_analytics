use crate::collector::CollectorCfg;
use analytics::collector;
use notify::{watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let path = "/home/sisso/games/aurora11/AuroraDBSaveBackup.db";

    println!("monitoring {}", path);

    collector::collect(CollectorCfg {
        db_path: path.to_string(),
        output_path: "data/aurora_dump.json".to_string(),
    })
    .unwrap();
}
