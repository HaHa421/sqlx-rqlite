use crate::connect::Connection;
use crate::error::RqliteError;
use crate::row::Row;
use crate::types::{parse_vec_types, Type};
use hyper::Request;
use serde::Deserialize;
use serde_json::{self, json};

/// Cursor
/// Holds all info when executing a command
#[derive(Debug)]
pub struct Cursor<'a> {
    /// Connection object
    connection: &'a mut Connection,
    //messages: Vec<String>,
    /// Last row in table
    lastrowid: Option<usize>,
    /// column and type
    description: Vec<(String, Type)>,
    /// Number of row read
    rownumber: usize,
    /// Number of rows affected/got from query
    rowcount: isize,
    //arraysize: isize,
    /// Rows result
    rows: Vec<Vec<serde_json::Value>>,
    //_column_type_cache: Option<String>
}

#[derive(Deserialize)]
struct ResultMap {
    error: Option<String>,
    columns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "parse_vec_types")]
    types: Option<Vec<Type>>,
    values: Option<Vec<Vec<serde_json::Value>>>,
    rows_affected: Option<usize>,
    last_insert_id: Option<usize>,
}

#[derive(Deserialize)]
struct QueryResult {
    results: Option<Vec<ResultMap>>,
}

impl<'l> Cursor<'l> {
    /// Create a new cursor
    /// Multiple cursor can be created for same connection
    /// Every query deletes last queries data
    pub(crate) fn new<'a>(connection: &'a mut Connection) -> Cursor {
        Cursor {
            connection,
            //messages: Vec::new(),
            lastrowid: None,
            description: Vec::new(),
            rownumber: 0,
            rowcount: -1,
            //arraysize: 1,
            rows: Vec::new(),
            //_column_type_cache: None
        }
    }

    /// Execute sql query
    /// ```
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    /// .connect().await?;
    /// let cur = conn.cursor();
    /// cur.execute("SELECT * FROM foo", par!()).await?;
    /// ```
    ///
    /// Returns RqliteError on error to handle exception explicitly
    pub async fn execute(
        &mut self,
        query: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<(), Box<RqliteError>> {
        let mut query_json;
        if params.is_empty() {
            query_json = json!([query]);
        } else {
            query_json = json!([[query]]);
            let v = query_json[0].as_array_mut().unwrap();
            for elem in params {
                v.push(elem);
            }
        }

        println!("query_json: {:?}", query_json);

        let mut req_builder = Request::builder().method("POST").uri(
            if query.starts_with("SELECT ") || query.starts_with("PRAGMA ") {
                "/db/query"
            } else {
                "/db/execute?transaction"
            },
        );
        req_builder = self
            .connection
            .auth(self.connection.base_headers(req_builder));
        let resp = self
            .connection
            .request(req_builder, Some(&query_json))
            .await?;

        let mut last_insert_id: Option<usize> = None;
        let mut rows_affected: isize = -1;

        let body = self.connection.read_body(resp).await?;
        let result: QueryResult = self.connection.body(&body).await?;
        match result.results {
            Some(res) => {
                rows_affected = 0;
                for item in res {
                    if item.error.is_some() {
                        return Err(Box::new(RqliteError::SqlError(item.error.unwrap())));
                    }

                    if item.rows_affected.is_some() {
                        rows_affected += item.rows_affected.unwrap() as isize;
                    }
                    if item.last_insert_id.is_some() {
                        last_insert_id = Some(item.last_insert_id.unwrap());
                    }
                    match item.columns {
                        Some(fields) => {
                            if item.types.is_some() {
                                let types = item.types.unwrap();
                                for i in 0..fields.len() {
                                    self.description.push((fields[i].clone(), types[i]));
                                }
                                if item.values.is_some() {
                                    self.rows = item.values.unwrap();
                                }
                            }
                        }
                        None => {
                            if query.starts_with("INSERT ") {
                                self.lastrowid = last_insert_id;
                            }
                        }
                    }
                }
            }
            None => {}
        }

        self.rownumber = 0;
        if query.starts_with("INSERT ")
            || query.starts_with("UPDATE ")
            || query.starts_with("DELETE ")
        {
            self.rowcount = rows_affected;
        } else {
            self.rowcount = self.rows.len() as isize;
        }
        Ok(())
    }

    /// Return number of rows read
    pub fn rows_read(&self) -> usize {
        self.rownumber
    }

    /// Returns number of rows affected/got from query
    /// -1 is returned if no query is done yet
    pub fn rows_affected(&self) -> isize {
        self.rowcount
    }

    /// Returns a slice of column and type tuples
    /// Empty slice returned when no select request is made
    pub fn description<'a>(&'a self) -> &'a [(String, Type)] {
        &self.description
    }

    /// Returns last row id
    /// Valid only for insert query
    pub fn last_row_id(&self) -> Option<usize> {
        self.lastrowid.clone()
    }

    /// Get next row
    /// ```
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    /// .connect().await?;
    /// let cur = conn.cursor();
    /// cur.execute("SELECT * FROM foo", par!())?;
    /// let mut i: i32;
    /// let mut str: String;
    /// while let Some(row) = cur.next_row() {
    ///     i   = row.get(0)?;
    ///     str = row.get(1)?;
    ///     println!("{} | {}", i, str);
    /// }
    /// // 0 | fiona
    /// // 1 | diego
    /// ```
    ///
    pub fn next_row(&mut self) -> Option<Row> {
        if self.rows.len() == 0 {
            return None;
        }
        self.rownumber += 1;
        Some(Row::new(self.rows.remove(0)))
    }
}
