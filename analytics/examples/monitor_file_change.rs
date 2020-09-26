use crate::collector::CollectorCfg;
use analytics::collector;

fn main() {
    let path = "/home/sisso/games/aurora11/AuroraDB.db";

    println!("monitoring {}", path);

    collector::collect(CollectorCfg {
        db_path: path.to_string(),
        output_path: "data/aurora_dump.json".to_string(),
    })
    .unwrap();
}
