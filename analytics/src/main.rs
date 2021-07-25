#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use analytics::dashboard::Dashboard;
use rocket::response::content;
use serde::Serialize;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct Generic500 {
    error: String,
}

impl From<std::io::Error> for Generic500 {
    fn from(e: std::io::Error) -> Self {
        Generic500 {
            error: format!("io error {:?}", e),
        }
    }
}

impl From<serde_json::Error> for Generic500 {
    fn from(e: serde_json::Error) -> Self {
        Generic500 {
            error: format!("parser error {:?}", e),
        }
    }
}

#[get("/")]
fn index() -> Result<content::Html<String>, Generic500> {
    let body = std::fs::read_to_string("analytics/resources/index.html")?;
    Ok(content::Html(body))
}

#[get("/data")]
fn data() -> Result<content::Json<String>, Generic500> {
    let body = std::fs::read_to_string("data/dashboard_data.json")?;
    Ok(content::Json(body))
}

fn get_data() -> Result<Dashboard, Generic500> {
    let body = std::fs::read_to_string("data/dashboard_data.json")?;
    let dashboard = serde_json::from_str(body.as_str())?;
    Ok(dashboard)
}

#[derive(Serialize, Debug)]
pub struct KeyValudDto<'a> {
    id: u32,
    name: &'a str,
}

#[get("/games")]
fn data_games() -> Result<content::Json<String>, Generic500> {
    let dashboard = get_data()?;
    let result: Vec<KeyValudDto> = dashboard
        .games
        .iter()
        .map(|game| KeyValudDto {
            id: game.game_id,
            name: game.game_name.as_str(),
        })
        .collect();

    let result_json = serde_json::to_string_pretty(&result)?;
    Ok(content::Json(result_json))
}

#[get("/games/<game_id>/populations")]
fn data_games_populations(game_id: u32) -> Result<content::Json<String>, Generic500> {
    let dashboard = get_data()?;
    let result: Vec<KeyValudDto> = dashboard
        .games
        .iter()
        .find(|game| game.game_id == game_id)
        .map(|game| {
            game.populations
                .iter()
                .map(|pop| KeyValudDto {
                    id: pop.population_id,
                    name: pop.population_name.as_str(),
                })
                .collect()
        })
        .unwrap_or(Vec::new());

    let result_json = serde_json::to_string_pretty(&result)?;
    Ok(content::Json(result_json))
}

#[get("/games/<game_id>/populations/<population_id>")]
fn data_games_populations_by_id(
    game_id: u32,
    population_id: u32,
) -> Result<content::Json<String>, Generic500> {
    let dashboard = get_data()?;

    let result = dashboard
        .games
        .iter()
        .find(|game| game.game_id == game_id)
        .iter()
        .flat_map(|game| {
            game.populations
                .iter()
                .find(|pop| pop.population_id == population_id)
        })
        .next()
        .unwrap();

    let result_json = serde_json::to_string_pretty(result)?;
    Ok(content::Json(result_json))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            data,
            data_games,
            data_games_populations,
            data_games_populations_by_id
        ],
    )
}
