use crate::collector::CollectorCfg;
use analytics::collector;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("path to aurora 4x must be provided");

    println!("monitoring {}", path);

    collector::collect(CollectorCfg {
        db_path: path.to_string(),
        dump_path: "data/aurora_dump.json".to_string(),
        dashboard_path: "data/dashboard_data.json".to_string(),
    })
    .unwrap();
}
