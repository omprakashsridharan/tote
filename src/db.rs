use crate::model::Tote;
use mysql::prelude::*;
use mysql::*;
pub struct ToteDb {
    conn: PooledConn,
}

impl ToteDb {
    pub fn initialise() -> Result<Self> {
        let url = "mysql://omprakash:password@localhost/tote";
        let pool = Pool::new(url)?;
        let conn = pool.get_conn()?;
        Ok(ToteDb { conn })
    }

    pub fn get_totes(&mut self) -> Result<Vec<Tote>> {
        let totes = self.conn.query_map(
            "select * from tote",
            |(id, note, created_at, updated_at)| Tote {
                id,
                note,
                created_at,
                updated_at,
            },
        );
        totes
    }

    pub fn add_tote(&mut self, note: String) -> Result<()> {
        let _row: Option<String> = self
            .conn
            .exec_first(
                "INSERT into tote (note) VALUES(:note)",
                params! {"note"=>note},
            )
            .unwrap();
        println!("Added tote");
        Ok(())
    }
}
