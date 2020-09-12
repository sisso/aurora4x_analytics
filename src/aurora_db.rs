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
        let sql = r#"select GameID, GameTime, GameName, LastViewed, StartYear, GameTime / (60 * 60 * 24 * 365) + StartYear
        from FCT_Game
        where LastViewed = 1.0;"#;

        let mut stmt = connection.prepare(sql).unwrap();
        let rows: Vec<u32> = stmt.query_map(&[], |row| {
            let game_id: u32 = row.get(0);
            println!("{:?}", game_id);
            game_id
        }).unwrap().collect::<Result<Vec<u32>, _>>().unwrap();

        println!("{:?}", rows);

       unimplemented!()
    }
}


