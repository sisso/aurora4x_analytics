use rusqlite::{Connection, Error as SQLError, OpenFlags};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FCTGame {
    pub game_id: u32,
    pub game_name: String,
    pub game_time: f64,
    pub start_year: u32,
    pub last_viewed: f64,
}

impl FCTGame {
    pub fn is_last(&self) -> bool {
        self.last_viewed > 0.0
    }

    pub fn year(&self) -> u32 {
        self.game_time as u32 / (60 * 60 * 24 * 365) + self.start_year
    }

    // pub fn month(&self) -> u32 {
    //     self.game_time as u32 / (60 * 60 * 24 * 365) + self.start_year
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FCTPopulation {
    pub population_id: u32,
    pub pop_name: String,
    pub fuel_stockpile: f64,
    pub maintenance_stockpile: f64,
    pub population: f64,
    pub duranium: f64,
    pub neutronium: f64,
    pub corbomite: f64,
    pub tritanium: f64,
    pub boronide: f64,
    pub mercassium: f64,
    pub vendarite: f64,
    pub sorium: f64,
    pub corundium: f64,
    pub gallicite: f64,
}

// #[test]
// fn test_fctgame_month() {
//     let seconds = 454_853_030;
//     let date = NaiveDateTime::from_timestamp(seconds, 0);
//     println!("{:?}", date);
//     panic!("{:?}", date);
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuroraGameData {
    pub game: FCTGame,
    pub race_id: u32,
    pub populations: Vec<FCTPopulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuroraData {
    pub games: Vec<AuroraGameData>,
}

#[derive(Debug)]
pub enum DbError {
    Generic(String),
}

#[derive(Debug)]
pub struct AuroraDb {
    path: PathBuf,
}

impl AuroraDb {
    pub fn new(path: &Path) -> Self {
        AuroraDb { path: path.into() }
    }

    pub fn fetch(&self) -> Result<AuroraData, DbError> {
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        let connection =
            Connection::open_with_flags(self.path.as_path(), flags).expect("fail to open database");

        let sql = r#"select GameID, GameName, GameTime, StartYear, LastViewed from FCT_Game where LastViewed = 1.0;"#;
        let mut stmt = connection.prepare(sql).unwrap();
        let games: Vec<FCTGame> = stmt
            .query_map(&[], |row| FCTGame {
                game_id: row.get(0),
                game_name: row.get(1),
                game_time: row.get(2),
                start_year: row.get(3),
                last_viewed: row.get(4),
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut games_data = vec![];

        for game in games {
            let sql = r#"select RaceID from FCT_Race where NPR = 0 and GameID = ?"#;
            let mut stmt = connection.prepare(sql).unwrap();
            let race_id: u32 = stmt.query_row(&[&game.game_id], |row| row.get(0)).unwrap();

            let sql = r#"select PopulationID, PopName, FuelStockpile, MaintenanceStockpile, Population, Duranium, Neutronium, Corbomite, Tritanium, Boronide, Mercassium, Vendarite, Sorium, Corundium, Gallicite, SystemBodyID
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

            games_data.push(AuroraGameData {
                game,
                race_id,
                populations,
            });
        }

        Ok(AuroraData { games: games_data })
    }
}
