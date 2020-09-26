#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> Result<content::Html<String>, std::io::Error> {
    let body = std::fs::read_to_string("analytics/resources/index.html")?;
    Ok(content::Html(body))
}

#[get("/data")]
fn data() -> Result<content::Json<String>, Box<dyn std::error::Error>> {
    let body = std::fs::read_to_string("data/dashboard_data.json")?;
    Ok(content::Json(body))
}

fn main() {
    rocket::ignite().mount("/", routes![index, data]).launch();
}
