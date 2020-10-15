use analytics::collector;

fn main() {
    let output_path = "./01.json";
    let paths = vec!["/home/sisso/games/aurora11/AuroraDB.db"];

    for path in paths {
        println!("processing {}", path);
        collector::append_output(path, output_path).unwrap();
    }
}
