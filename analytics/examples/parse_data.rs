use analytics::db::Db;

fn main() {
    let db = Db::load(&"data/01.log").unwrap();

    let json = serde_json::to_string_pretty(&db.get_data().unwrap()).unwrap();

    std::fs::write("/tmp/01.json", json);
}
