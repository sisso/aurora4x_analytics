use analytics::db::{Dashboard, DashboardDb};

fn main() {
    let dashboard = DashboardDb::load_aurora_dump(&"data-samples/aurora_dump.json").unwrap();
    DashboardDb::save(&dashboard, "data-samples/dashboard_data.json").unwrap();
}
