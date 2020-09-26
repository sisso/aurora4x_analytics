use analytics::db::{Dashboard, DashboardDb};

fn main() {
    let dashboard = DashboardDb::load_aurora_dump(&"data/aurora_dump.json").unwrap();
    DashboardDb::save(&dashboard, "data/dashboard_data.json").unwrap();
}
