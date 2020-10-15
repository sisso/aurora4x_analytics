use analytics::collector;

fn main() {
    let base_path = std::env::args()
        .nth(1)
        .expect("path to aurora 4x must be provided");

    let path = format!("{}/AuroraDB.db", base_path);

    let output_path = "data/aurora_dump.json";
    let dashboard_path = "data/dashboard_data.json";

    println!("processing {}", path);
    collector::append_output(path.as_str(), output_path).unwrap();
    collector::convert_into_dashboard(&output_path, &dashboard_path).unwrap();
}
