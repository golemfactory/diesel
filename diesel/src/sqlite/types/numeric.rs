#![cfg(feature = "bigdecimal")]

extern crate bigdecimal;
use sqlite::types::numeric::bigdecimal::FromPrimitive;
use self::bigdecimal::BigDecimal;

use deserialize::{self, FromSql};
use sql_types::{Double, Numeric};
use sqlite::connection::SqliteValue;
use sqlite::Sqlite;

impl FromSql<Numeric, Sqlite> for BigDecimal {
    fn from_sql(bytes: Option<&SqliteValue>) -> deserialize::Result<Self> {
        let data = <f64 as FromSql<Double, Sqlite>>::from_sql(bytes)?;
        let bd = BigDecimal::from_f64(data).ok_or_default();
        Ok(bd)
    }
}
