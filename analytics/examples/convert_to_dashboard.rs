use analytics::db::DashboardDb;

fn main() {
    let db = DashboardDb::load(&"data/aurora_dump.json").unwrap();

    let json = serde_json::to_string_pretty(&db.get_data().unwrap()).unwrap();
    std::fs::write("data/dashboard_data.json", json).unwrap();
}
