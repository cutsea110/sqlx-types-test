use chrono::NaiveTime;
use core::ops::Bound;
use serde::{Deserialize, Serialize};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgTypeInfo, PgTypeKind, Postgres};
use sqlx::{Decode, Encode, Type};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct PgTimeRange {
    pub start: Bound<NaiveTime>,
    pub end: Bound<NaiveTime>,
}

impl Type<Postgres> for PgTimeRange {
    fn type_info() -> PgTypeInfo {
        panic!("TODO")
    }
}

impl<'r> Decode<'r, Postgres> for PgTimeRange {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        panic!("TODO")
    }
}

impl<'q> Encode<'q, Postgres> for PgTimeRange {
    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        panic!("TODO")
    }
}
