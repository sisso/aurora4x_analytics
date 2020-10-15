use analytics::collector;

fn main() {
    let output_path = "data-samples/aurora_dump.json";
    let dashboard_path = "data-samples/dashboard_data.json";
    let paths = vec!["/home/sisso/games/aurora11/AuroraDB.db"];

    for path in paths {
        println!("processing {}", path);
        collector::append_output(path, output_path).unwrap();
        collector::convert_into_dashboard(&output_path, &dashboard_path).unwrap();
    }
}
