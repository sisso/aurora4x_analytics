use crate::aurora_db::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct CollectorCfg {
    pub db_path: String,
    pub output_path: String,
}

#[derive(Debug)]
pub enum CollectorError {}

pub fn collect(cfg: CollectorCfg) -> Result<(), CollectorError> {
    watch(&cfg.db_path, || {
        append_output(&cfg.db_path, &cfg.output_path).unwrap();
    })
}

pub fn append_output(db_path: &str, output_path: &str) -> Result<(), CollectorError> {
    let aurora_db = AuroraDb::new(&PathBuf::from(db_path));
    let data = aurora_db.fetch().unwrap();

    let json = serde_json::to_string(&data).unwrap();

    {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(output_path)
            .unwrap();

        writeln!(file, "{}", json).unwrap();
    }

    println!("data updated at {}", output_path);

    Ok(())
}

fn watch<F>(db_path: &str, callback: F) -> Result<(), CollectorError>
where
    F: Fn(),
{
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(db_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::NoticeWrite(_)) => {
                // lets wait some time after the notification to check if we have access
                sleep(Duration::from_secs(1));
                callback()
            }
            Ok(_) => {}
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
