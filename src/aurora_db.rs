use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{Connection, Error as SQLError, OpenFlags};
use std::path::PathBuf;
use time::Timespec;

// mod errors {
//     use error_chain::error_chain;
//
//     error_chain! {
//         foreign_links! {
//             SQLError
//         }
//     }
// }

// pub use errors::*;

#[derive(Debug, Clone)]
pub struct FCTGame {
    game_id: u32,
    game_name: String,
    game_time: f64,
    start_year: u32,
}

#[derive(Debug, Clone)]
pub struct FCTPopulation {
    population_id: u32,
    pop_name: String,
    fuel_stockpile: f64,
    maintenance_stockpile: f64,
    population: f64,
    duranium: f64,
    neutronium: f64,
    corbomite: f64,
    tritanium: f64,
    boronide: f64,
    mercassium: f64,
    vendarite: f64,
    sorium: f64,
    corundium: f64,
    gallicite: f64,
}

impl FCTGame {
    pub fn year(&self) -> u32 {
        self.game_time as u32 / (60 * 60 * 24 * 365) + self.start_year
    }

    // pub fn month(&self) -> u32 {
    //     self.game_time as u32 / (60 * 60 * 24 * 365) + self.start_year
    // }
}

// #[test]
// fn test_fctgame_month() {
//     let seconds = 454_853_030;
//     let date = NaiveDateTime::from_timestamp(seconds, 0);
//     println!("{:?}", date);
//     panic!("{:?}", date);
// }

#[derive(Debug, Clone)]
pub struct Data {}

#[derive(Debug)]
pub enum DbError {
    Generic(String),
}

#[derive(Debug)]
pub struct AuroraDb {
    path: PathBuf,
}

impl AuroraDb {
    pub fn new(path: PathBuf) -> Self {
        AuroraDb { path }
    }

    pub fn fetch(&self) -> Result<Data, DbError> {
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        let connection =
            Connection::open_with_flags(self.path.as_path(), flags).expect("fail to open database");

        let sql =
            r#"select GameID, GameName, GameTime, StartYear from FCT_Game where LastViewed = 1.0;"#;
        let mut stmt = connection.prepare(sql).unwrap();
        let game = stmt
            .query_row(&[], |row| FCTGame {
                game_id: row.get(0),
                game_name: row.get(1),
                game_time: row.get(2),
                start_year: row.get(3),
            })
            .unwrap();
        println!("{:?}", game);

        let sql = r#"select RaceID from FCT_Race where NPR = 0 and GameID = ?"#;
        let mut stmt = connection.prepare(sql).unwrap();
        let race_id: u32 = stmt.query_row(&[&game.game_id], |row| row.get(0)).unwrap();
        println!("race_id: {}", race_id);

        let sql = r#"select PopulationID, PopName, FuelStockpile, MaintenanceStockpile, Population, Duranium, Neutronium, Corbomite, Tritanium, Boronide, Mercassium, Vendarite, Sorium, Corundium, Gallicite
from FCT_Population
where RaceID = ?"#;
        let mut stmt = connection.prepare(sql).unwrap();
        let populations: Vec<FCTPopulation> = stmt
            .query_map(&[&race_id], |row| FCTPopulation {
                population_id: row.get(0),
                pop_name: row.get(1),
                fuel_stockpile: row.get(2),
                maintenance_stockpile: row.get(3),
                population: row.get(4),
                duranium: row.get(5),
                neutronium: row.get(6),
                corbomite: row.get(7),
                tritanium: row.get(8),
                boronide: row.get(9),
                mercassium: row.get(10),
                vendarite: row.get(11),
                sorium: row.get(12),
                corundium: row.get(13),
                gallicite: row.get(14),
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        for p in populations {
            println!("{:?}", p);
        }

        Ok(Data {})
    }
}
