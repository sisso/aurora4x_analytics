use rusqlite::{Connection, OpenFlags};
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
    pub system_id: Option<u32>,
    pub system_body_id: Option<u32>,
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
    /// optional as old don't have this field
    pub uridium: Option<f64>,
    pub gallicite: f64,
    /// optional as old don't have this field
    pub minerals: Option<Vec<FCTMineralDeposit>>,
}

// #[test]
// fn test_fctgame_month() {
//     let seconds = 454_853_030;
//     let date = NaiveDateTime::from_timestamp(seconds, 0);
//     println!("{:?}", date);
//     panic!("{:?}", date);
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FCTRace {
    pub race_id: u32,
    pub wealth: f64,
    pub annual_wealth: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FCTMineralDeposit {
    pub material_id: u32,
    pub amount: f64,
    pub acc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuroraGameData {
    pub game: FCTGame,
    pub race_id: u32,
    pub race: Option<FCTRace>,
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

        let games = self.fetch_games(&connection);

        let mut games_data = vec![];

        for game in games {
            let race = self.fetch_race(&connection, game.game_id);
            let mut populations = self.fetch_populations(&connection, race.race_id);

            for pop in populations.iter_mut() {
                let minerals = self.fetch_pop_minerals(
                    &connection,
                    game.game_id,
                    pop.system_id.expect("system_id not returned"),
                    pop.system_body_id.expect("system_body_id not returned"),
                );

                pop.minerals = Some(minerals);
            }

            games_data.push(AuroraGameData {
                game,
                race_id: race.race_id,
                populations,
                race: Some(race),
            });
        }

        Ok(AuroraData { games: games_data })
    }

    fn fetch_race(&self, connection: &Connection, game_id: u32) -> FCTRace {
        let sql = r#"select RaceID, WealthPoints, AnnualWealth from FCT_Race where NPR = 0 and GameID = ?"#;
        let mut stmt = connection.prepare(sql).unwrap();
        stmt.query_row(&[&game_id], |row| FCTRace {
            race_id: row.get(0),
            wealth: row.get(1),
            annual_wealth: row.get(2),
        })
        .unwrap()
    }

    fn fetch_pop_minerals(
        &self,
        connection: &Connection,
        game_id: u32,
        system_id: u32,
        system_body_id: u32,
    ) -> Vec<FCTMineralDeposit> {
        let sql = r#"select MaterialID, Amount, Accessibility from FCT_MineralDeposit where GameID = ? and SystemID = ? and SystemBodyID = ?"#;
        let mut stmt = connection.prepare(sql).unwrap();
        stmt.query_map(&[&game_id, &system_id, &system_body_id], |row| {
            FCTMineralDeposit {
                material_id: row.get(0),
                amount: row.get(1),
                acc: row.get(2),
            }
        })
        .unwrap()
        .collect::<Result<Vec<FCTMineralDeposit>, _>>()
        .unwrap()
    }

    // TODO: this should be probably in when reading db and converting to dashboard
    // fn fetch_and_merge_pop_minerals(
    //     &self,
    //     connection: &Connection,
    //     game_id: u32,
    //     system_id: u32,
    //     system_body_id: u32,
    //     pop: &FCTPopulation,
    // ) -> Vec<PopMinerals> {
    //     let sql = r#"select MaterialID, Amount, Accessibility from FCT_MineralDeposit where GameID = ? and SystemID = ? and SystemBodyID = ?"#;
    //     let mut stmt = connection.prepare(sql).unwrap();
    //     let minerals: HashMap<u32, (f64, f64)> = stmt
    //         .query_map(&[&game_id, &system_id, &system_body_id], |row| {
    //             (row.get(0), (row.get(1), row.get(2)))
    //         })
    //         .unwrap()
    //         .collect::<Result<HashMap<_, _>, _>>()
    //         .unwrap();
    //
    //     let mut index: Vec<(&str, f64, Option<(f64, f64)>)> = vec![];
    //
    //     macro_rules! append_index {
    //         ($field:tt, $index:tt) => {
    //             index.push((
    //                 std::stringify!($field),
    //                 pop.$field,
    //                 minerals.get(&$index).cloned(),
    //             ));
    //         };
    //         ($field:tt, $index:tt, optional) => {
    //             index.push((
    //                 std::stringify!($field),
    //                 pop.$field.clone().unwrap_or(0.0),
    //                 minerals.get(&$index).cloned(),
    //             ));
    //         };
    //     };
    //
    //     append_index!(duranium, 1);
    //     append_index!(neutronium, 2);
    //     append_index!(corbomite, 3);
    //     append_index!(tritanium, 4);
    //     append_index!(boronide, 5);
    //     append_index!(mercassium, 6);
    //     append_index!(vendarite, 7);
    //     append_index!(sorium, 8);
    //     append_index!(uridium, 9, optional);
    //     append_index!(corundium, 10);
    //     append_index!(gallicite, 11);
    //
    //     index
    //         .into_iter()
    //         .map(|(field, stock, pop_minerals)| PopMinerals {
    //             name: field.to_string(),
    //             stock: stock,
    //             available: pop_minerals.map(|i| i.0).unwrap_or(0.0),
    //             accessibility: pop_minerals.map(|i| i.1).unwrap_or(0.0),
    //         })
    //         .collect()
    // }

    fn fetch_populations(&self, connection: &Connection, race_id: u32) -> Vec<FCTPopulation> {
        let sql = r#"select PopulationID, PopName, FuelStockpile, 
                                  MaintenanceStockpile, Population, Duranium, 
                                  Neutronium, Corbomite, Tritanium, Boronide, 
                                  Mercassium, Vendarite, Sorium, Corundium,
                                  Uridium,  Gallicite, SystemID, 
                                  SystemBodyID
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
                uridium: row.get(14),
                gallicite: row.get(15),
                system_id: row.get(16),
                system_body_id: row.get(17),
                minerals: None,
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        populations
    }

    fn fetch_games(&self, connection: &Connection) -> Vec<FCTGame> {
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
        games
    }
}
