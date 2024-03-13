use crate::error::RqliteError;
use crate::type_info::DataType;
use crate::RqliteColumn;
use crate::{
    Rqlite, RqliteConnection, RqliteQueryResult, RqliteRow, RqliteStatement, RqliteTypeInfo,
    RqliteValue,
};
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::pin_mut;
use futures_util::TryStreamExt;
use sqlx_core::describe::Describe;
use sqlx_core::error::Error;
use sqlx_core::executor::{Execute, Executor};
use sqlx_core::ext::ustr::UStr;
use sqlx_core::try_stream;
use sqlx_core::Either;

impl<'c> Executor<'c> for &'c mut RqliteConnection {
    type Database = Rqlite;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxStream<'e, Result<Either<RqliteQueryResult, RqliteRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        let sql = query.sql();
        let arguments = query.take_arguments();
        //let persistent = query.persistent() && arguments.is_some();

        //let args = Vec::with_capacity(arguments.len());

        Box::pin(try_stream! {
          let cursor = self.inner.execute(sql, match arguments {
            Some(arguments)=>arguments.values,
            _=>vec![],
          }).await;
          match cursor {
            Ok(cursor)=> {
              //println!("{}:({})",file!(),line!());

              pin_mut!(cursor);

              while let Some(row) = cursor.next_row() {
                let size = row.column_count();
                let mut values = Vec::with_capacity(size);
                let mut columns = Vec::with_capacity(size);
                //let mut column_names = Vec::with_capacity(size);
                  for (i,value) in row.row.into_iter().enumerate() {
                    values.push(RqliteValue::new(value,RqliteTypeInfo(DataType::Null)));
                    columns.push(RqliteColumn{
                      name : UStr::from(""),
                      ordinal: i,
                      type_info: RqliteTypeInfo(DataType::Null),
                    });
                  }
                  let row=Either::Right(RqliteRow {
                    values: values.into_boxed_slice(),
                    columns: columns.into(),
                    column_names : Default::default(),
                  });
                  r#yield!(row);
              }
              Ok(())
            }
            Err(err)=> {
              Err(RqliteError{
                inner: err,
              }.into())
            }
          }
        })
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Option<RqliteRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        let mut s = self.fetch_many(query);

        Box::pin(async move {
            while let Some(v) = s.try_next().await? {
                if let Either::Right(r) = v {
                    return Ok(Some(r));
                }
            }

            Ok(None)
        })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        _sql: &'q str,
        _parameters: &[RqliteTypeInfo],
    ) -> BoxFuture<'e, Result<RqliteStatement<'q>, Error>>
    where
        'c: 'e,
    {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "prepare_with not supported",
            )))
        })
    }
    #[doc(hidden)]
    fn describe<'e, 'q: 'e>(self, _sql: &'q str) -> BoxFuture<'e, Result<Describe<Rqlite>, Error>>
    where
        'c: 'e,
    {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "describe not supported",
            )))
        })
    }
}
