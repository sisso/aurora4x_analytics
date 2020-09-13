use rusqlite::{Connection, Error as SQLError, OpenFlags};
use time::Timespec;
use std::path::PathBuf;

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

impl FCTGame {
    pub fn year(&self) -> u32 {
        self.game_time as u32 / (60 * 60 * 24 * 365) + self.start_year
    }
}

#[derive(Debug, Clone)]
pub struct Data {

}

#[derive(Debug)]
pub enum DbError {
    Generic(String)
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
        let connection = Connection::open_with_flags(self.path.as_path(), flags).expect("fail to open database");

        let sql = r#"select GameID, GameName, GameTime, StartYear from FCT_Game where LastViewed = 1.0;"#;
        let mut stmt = connection.prepare(sql).unwrap();
        let rows: Vec<_> = stmt.query_map(&[], |row| {
            FCTGame {
                game_id: row.get(0),
                game_name: row.get(1),
                game_time: row.get(2),
                start_year:row.get(3),
            }
        }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();

        println!("{:?}", rows);


        Ok(Data {

        })
    }
}


