use rqlite::ConnectOptions;
use rqlite::Scheme;

//use rqlite::par;
//use rqlite::types::ToJsonValue;
use chrono::{DateTime, Utc};
use rqlite::types::BoxDynError;


//
// let mut conn = ConnectOptions::new("my.node.local", 4001)
//     .scheme(Scheme::HTTPS)
//     .user("root")
//     .pass("root")
//		.connect().await?;
//	conn.execute("SELECT * FROM foo where id = ?;", par!(1)).await?;

fn main() -> Result<(), BoxDynError> {
    let rt = Runtime::new()?;
    let r = rt.block_on(async {
        let mut conn = ConnectOptions::new("localhost", 4001)
            .scheme(Scheme::HTTP)
            //.user("root")
            //.pass("root")
            .connect()
            .await?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        )",
            vec![],
        )
        .await?;
        let mut cursor = conn.execute("SELECT * FROM user;", vec![]).await?;

        while let Some(row) = cursor.next_row() {
            println!("row: {:?}", row);
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            //println!("{} : {}", id,name/*, birth_date*/);
            println!("{} : {} | {:?}", id, name, birth_date);
        }
        Result::<_, BoxDynError>::Ok(())
    });

    Ok(r?)
}
