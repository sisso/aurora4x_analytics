use crate::aurora_db::AuroraData;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
pub enum DbError {}

#[derive(Debug)]
pub struct DashboardDb {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalValue {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardField {
    pub name: String,
    pub historical: Vec<HistoricalValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPopulation {
    pub population_id: u32,
    pub population_name: String,
    pub fields: Vec<DashboardField>,
}

impl DashboardPopulation {
    pub fn get_field(&self, name: &str) -> &DashboardField {
        self.fields
            .iter()
            .find(|f| f.name.as_str() == name)
            .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDashboard {
    pub game_id: u32,
    pub game_name: String,
    pub fields: Vec<DashboardField>,
    pub populations: Vec<DashboardPopulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub games: Vec<GameDashboard>,
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard { games: Vec::new() }
    }
}

impl Dashboard {
    pub fn append(&mut self, adata: &AuroraData) {
        for agame in &adata.games {
            let date = agame.game.game_time;

            let game_data: &mut GameDashboard = match self
                .games
                .iter_mut()
                .find(|g| g.game_id == agame.game.game_id)
            {
                None => {
                    self.games.push(GameDashboard {
                        game_id: agame.game.game_id,
                        game_name: agame.game.game_name.clone(),
                        fields: vec![],
                        populations: vec![],
                    });

                    self.games.last_mut().unwrap()
                }

                Some(game_data) => game_data,
            };

            for ap in &agame.populations {
                let pop: &mut DashboardPopulation = match game_data
                    .populations
                    .iter_mut()
                    .find(|i| i.population_id == ap.population_id)
                {
                    None => {
                        game_data.populations.push(DashboardPopulation {
                            population_id: ap.population_id,
                            population_name: ap.pop_name.clone(),
                            fields: vec![],
                        });

                        game_data.populations.last_mut().unwrap()
                    }

                    Some(pop) => pop,
                };

                macro_rules! append_field {
                    ($f:tt) => {
                        Dashboard::append_field(pop, date, std::stringify!($f), ap.$f);
                    };
                    ($f:tt, option) => {
                        Dashboard::append_field(
                            pop,
                            date,
                            std::stringify!($f),
                            ap.$f.unwrap_or(0.0),
                        );
                    };
                }

                append_field!(fuel_stockpile);
                append_field!(maintenance_stockpile);
                append_field!(population);
                append_field!(duranium);
                append_field!(neutronium);
                append_field!(corbomite);
                append_field!(tritanium);
                append_field!(boronide);
                append_field!(mercassium);
                append_field!(vendarite);
                append_field!(sorium);
                append_field!(uridium, option);
                append_field!(corundium);
                append_field!(gallicite);
            }
        }
    }

    fn append_field(pop: &mut DashboardPopulation, date: f64, fieldname: &str, value: f64) {
        let f: &mut DashboardField =
            match pop.fields.iter_mut().find(|i| i.name.as_str() == fieldname) {
                None => {
                    pop.fields.push(DashboardField {
                        name: fieldname.to_string(),
                        historical: vec![],
                    });

                    pop.fields.last_mut().unwrap()
                }

                Some(f) => f,
            };

        // horrible sorting after insert, but will be noticed?
        f.historical.push(HistoricalValue { x: date, y: value });
        f.historical
            .sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    }
}

impl DashboardDb {
    pub fn load_aurora_dump(path: &str) -> Result<Dashboard, DbError> {
        // TODO: add error handling
        let mut db_data = Dashboard::new();
        let file = File::open(&path).unwrap();
        for line in std::io::BufReader::new(file).lines() {
            let data: AuroraData = serde_json::from_str(&line.unwrap()).unwrap();
            db_data.append(&data);
        }
        Ok(db_data)
    }

    pub fn save(dashboard: &Dashboard, path: &str) -> Result<(), DbError> {
        // TODO: add error handling
        let json = serde_json::to_string_pretty(dashboard).unwrap();
        std::fs::write(path, json).unwrap();
        println!("dashboard updated at {}", path);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aurora_db::{AuroraGameData, FCTGame, FCTMineralDeposit, FCTPopulation, FCTRace};

    fn game_data_old(time: f64) -> AuroraGameData {
        AuroraGameData {
            game: FCTGame {
                game_id: 0,
                game_name: "Game 01".to_string(),
                game_time: time,
                start_year: 2,
                last_viewed: 1.0,
            },
            race_id: 4,
            populations: vec![FCTPopulation {
                population_id: 1,
                system_id: None,
                system_body_id: None,
                pop_name: "Pop 1".to_string(),
                fuel_stockpile: 1.0,
                maintenance_stockpile: 2.0,
                population: 3.0,
                duranium: 4.0,
                neutronium: 5.0,
                corbomite: 6.0,
                tritanium: 7.0,
                boronide: 8.0,
                mercassium: 9.0,
                vendarite: 10.0,
                sorium: 11.0,
                corundium: 11.0,
                uridium: None,
                gallicite: 12.0,
                minerals: None,
            }],
            race: None,
        }
    }

    fn game_data(time: f64) -> AuroraGameData {
        AuroraGameData {
            game: FCTGame {
                game_id: 0,
                game_name: "Game 01".to_string(),
                game_time: time,
                start_year: 2,
                last_viewed: 1.0,
            },
            race_id: 4,
            populations: vec![FCTPopulation {
                population_id: 1,
                system_id: None,
                system_body_id: None,
                pop_name: "Pop 1".to_string(),
                fuel_stockpile: 1.0,
                maintenance_stockpile: 2.0,
                population: 3.0,
                duranium: 4.0,
                neutronium: 5.0,
                corbomite: 6.0,
                tritanium: 7.0,
                boronide: 8.0,
                mercassium: 9.0,
                vendarite: 10.0,
                sorium: 11.0,
                corundium: 11.0,
                uridium: None,
                gallicite: 12.0,
                minerals: Some(vec![
                    FCTMineralDeposit {
                        material_id: 1,
                        amount: 43.0,
                        acc: 0.3,
                    },
                    FCTMineralDeposit {
                        material_id: 2,
                        amount: 430.0,
                        acc: 0.8,
                    },
                ]),
            }],
            race: Some(FCTRace {
                race_id: 4,
                wealth: 40.0,
                annual_wealth: 10.0,
            }),
        }
    }

    #[test]
    fn dbdata_append_test() {
        let mut db_data = Dashboard::new();
        let aurora_data = AuroraData {
            games: vec![game_data(1.0)],
        };

        db_data.append(&aurora_data);

        assert_eq!(db_data.games.len(), 1);
        assert_eq!(db_data.games[0].game_id, 0);
        assert_eq!(db_data.games[0].game_name, "Game 01".to_string());
        assert_eq!(db_data.games[0].populations.len(), 1);
        assert_eq!(db_data.games[0].populations[0].population_name, "Pop 1");
        assert_eq!(db_data.games[0].populations[0].fields.len(), 13);
        assert_eq!(
            db_data.games[0].populations[0]
                .get_field("population")
                .historical[0]
                .y,
            3.0
        );
    }

    #[test]
    fn dbdata_should_keep_historical_data_sorted_by_time() {
        let mut db_data = Dashboard::new();
        let aurora_data = AuroraData {
            games: vec![game_data_old(5.0), game_data(1.0), game_data(3.0)],
        };

        db_data.append(&aurora_data);

        // game fields
        assert_eq!(db_data.games[0].fields.len(), 2);
        assert_eq!(db_data.games[0].fields[0].name, "wealth");
        assert_eq!(db_data.games[0].fields[0].historical.len(), 2);
        assert_eq!(db_data.games[0].fields[0].historical[0].y, 40.0);
        assert_eq!(db_data.games[0].fields[0].historical[1].y, 40.0);
        assert_eq!(db_data.games[0].fields[1].name, "annual_wealth");
        assert_eq!(db_data.games[0].fields[0].historical.len(), 2);

        // populations
        assert_eq!(
            db_data.games[0].populations[0].fields[0].historical.len(),
            3
        );

        let historical = &db_data.games[0].populations[0].fields[0].historical;
        assert!(
            (historical[0].x - 1.0).abs() < 0.1,
            format!("fail at {}", historical[0].x)
        );
        assert!(
            (historical[1].x - 3.0).abs() < 0.1,
            format!("fail at {}", historical[1].x)
        );
        assert!(
            (historical[2].x - 5.0).abs() < 0.1,
            format!("fail at {}", historical[2].x)
        );
    }
}
