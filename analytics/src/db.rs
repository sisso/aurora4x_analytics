use crate::aurora_db::{AuroraData, AuroraGameData};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
pub enum DbError {}

#[derive(Debug)]
pub struct Db {
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalValue {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDbData {
    name: String,
    historical: Vec<HistoricalValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationDbData {
    population_id: u32,
    fields: Vec<FieldDbData>,
}

impl PopulationDbData {
    pub fn get_field(&self, name: &str) -> &FieldDbData {
        self.fields
            .iter()
            .find(|f| f.name.as_str() == name)
            .unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDbData {
    game_id: u32,
    game_name: String,
    populations: Vec<PopulationDbData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbData {
    games: Vec<GameDbData>,
}

impl DbData {
    pub fn new() -> Self {
        DbData { games: Vec::new() }
    }
}

impl DbData {
    pub fn append(&mut self, adata: &AuroraData) {
        for agame in &adata.games {
            let date = agame.game.game_time;

            let game_data: &mut GameDbData = match self
                .games
                .iter_mut()
                .find(|g| g.game_id == agame.game.game_id)
            {
                None => {
                    self.games.push(GameDbData {
                        game_id: agame.game.game_id,
                        game_name: agame.game.game_name.clone(),
                        populations: vec![],
                    });

                    self.games.last_mut().unwrap()
                }

                Some(game_data) => game_data,
            };

            for ap in &agame.populations {
                let pop: &mut PopulationDbData = match game_data
                    .populations
                    .iter_mut()
                    .find(|i| i.population_id == ap.population_id)
                {
                    None => {
                        game_data.populations.push(PopulationDbData {
                            population_id: ap.population_id,
                            fields: vec![],
                        });

                        game_data.populations.last_mut().unwrap()
                    }

                    Some(pop) => pop,
                };

                macro_rules! append_field {
                    ($f:tt) => {
                        DbData::append_field(pop, date, std::stringify!($f), ap.$f);
                    };
                };

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
                append_field!(corundium);
                append_field!(gallicite);
            }
        }
    }

    fn append_field(pop: &mut PopulationDbData, date: f64, fieldname: &str, value: f64) {
        let f: &mut FieldDbData = match pop.fields.iter_mut().find(|i| i.name.as_str() == fieldname)
        {
            None => {
                pop.fields.push(FieldDbData {
                    name: fieldname.to_string(),
                    historical: vec![],
                });

                pop.fields.last_mut().unwrap()
            }

            Some(f) => f,
        };

        f.historical.push(HistoricalValue { x: date, y: value });
    }
}

impl Db {
    pub fn load(path: &str) -> Result<Self, DbError> {
        Ok(Db {
            path: path.to_string(),
        })
    }

    pub fn get_data(&self) -> Result<DbData, DbError> {
        let mut db_data = DbData::new();

        let file = File::open(&self.path).unwrap();
        for line in std::io::BufReader::new(file).lines() {
            let data: AuroraData = serde_json::from_str(&line.unwrap()).unwrap();
            db_data.append(&data);
        }

        Ok(db_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aurora_db::{FCTGame, FCTPopulation};

    #[test]
    fn dbdata_append_test() {
        let mut db_data = DbData::new();
        let aurora_data = AuroraData {
            games: vec![AuroraGameData {
                game: FCTGame {
                    game_id: 0,
                    game_name: "Game 01".to_string(),
                    game_time: 1.0,
                    start_year: 2,
                    last_viewed: 1.0,
                },
                race_id: 4,
                populations: vec![FCTPopulation {
                    population_id: 1,
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
                    gallicite: 12.0,
                }],
            }],
        };

        db_data.append(&aurora_data);

        println!("{}", serde_json::to_string_pretty(&db_data).unwrap());

        assert_eq!(db_data.games.len(), 1);
        assert_eq!(db_data.games[0].game_id, 0);
        assert_eq!(db_data.games[0].game_name, "Game 01".to_string());
        assert_eq!(db_data.games[0].populations.len(), 1);
        assert_eq!(db_data.games[0].populations[0].fields.len(), 13);
        assert_eq!(
            db_data.games[0].populations[0]
                .get_field("population")
                .historical[0]
                .y,
            3.0
        );
    }
}
