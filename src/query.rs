use anyhow::Error;
use dotenv::dotenv;
use sqlx::{mysql::MySql, Executor, MySqlPool};

#[allow(unused_imports)]
use std::time::Instant;

pub fn run_query(query: &str) -> Result<Vec<Vec<String>>, Error> {
    let result = vec![
        vec![1.to_string(), 2.to_string(), 3.to_string()],
        vec![4.to_string(), 5.to_string(), 6.to_string()],
    ];
    Ok(result)
}

/// Maximum number of rows fetched with one row set. Fetching batches of rows is usually much
/// faster than fetching individual rows.

pub async fn establish_connection() -> std::result::Result<MySqlPool, Box<dyn std::error::Error>> {
    let database_url = "mysql://user:password@mysql:3306/db";
    let pool = MySqlPool::connect_lazy(database_url)?;
    let ret = pool.execute("SHOW databases;").await.unwrap();

    Ok(pool)
}

// pub struct MyConnection<'conn> {
//     pub conn: Connection<'conn>,
// }

// impl MyConnection<'static> {
//     pub fn new(env: &'static Environment) -> Self {
//         let connection_string = "
//             Driver={ODBC Driver 17 for SQL Server};\
//             Server=mssql;\
//             UID=SA;\
//             PWD=Passw0rd;\
//         ";
//         let connection = env
//             .connect_with_connection_string(connection_string, ConnectionOptions::default())
//             .expect("cannot connect to database");
//         MyConnection { conn: connection }
//     }

//     pub fn run_query(&self, query: &str) -> Result<(), Error> {
//         self.conn.execute(query, ())?;
//         Ok(())
//     }

//     pub fn execute_query(&self, query: &str) -> Result<Vec<Vec<String>>, Error> {
//         let start = Instant::now();
//         let mut result = vec![];
//         match self.conn.execute(query, ())? {
//             Some(mut cursor) => {
//                 // Write the column names to stdout
//                 let headline: Vec<String> = cursor.column_names()?.collect::<Result<_, _>>()?;
//                 result.push(headline);
//                 let mut buffers =
//                     TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096)).unwrap();
//                 let mut row_set_cursor = cursor.bind_buffer(&mut buffers).unwrap();

//                 while let Some(batch) = row_set_cursor.fetch()? {
//                     // Within a batch, iterate over every row
//                     for row_index in 0..batch.num_rows() {
//                         // Within a row iterate over every column
//                         let mut row = vec![];
//                         let record = (0..batch.num_cols())
//                             .map(|col_index| batch.at(col_index, row_index).unwrap_or(&[]));
//                         for v in record {
//                             row.push(std::str::from_utf8(v).unwrap().to_string());
//                         }
//                         result.push(row.clone());
//                     }
//                 }
//                 result.push(vec![format!(
//                     "<{} msec.>",
//                     (Instant::now() - start).as_millis()
//                 )
//                 .to_string()]);
//             }
//             // Err(E) => {
//             //     result.push(vec![format!("{}", E)]);
//             // }
//             None => {
//                 result.push(vec!["[ok]".to_string()]);
//             }
//         }
//         // let end = start.elapsed();
//         // eprintln!(
//         //     "elapsed time: {}.{:03} sec.",
//         //     end.as_secs(),
//         //     end.as_millis() % 1000
//         // );
//         Ok(result)
//     }
// }
