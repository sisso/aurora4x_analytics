use analytics::collector;

fn main() {
    let output_path = "data/aurora_dump.json";
    let dashboard_path = "data/dashboard_data.json";
    let paths = vec!["/home/sisso/home/shared/aurora11/AuroraDB.db"];

    for path in paths {
        println!("processing {}", path);
        collector::append_output(path, output_path).unwrap();
        collector::convert_into_dashboard(&output_path, &dashboard_path).unwrap();
    }
}
