use std::{
    fmt::{self, Debug},
    ops::Deref,
};

use rusqlite::{
    Result, ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};
use uuid::Uuid;

pub struct DbUuid(Uuid);

impl ToSql for DbUuid {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for DbUuid {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(value.as_str()?.parse::<Uuid>().unwrap().into())
    }
}

impl Debug for DbUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl Deref for DbUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for DbUuid {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
