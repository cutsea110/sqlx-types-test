use anyhow::Result;
use bigdecimal::BigDecimal;
use bit_vec::BitVec;
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono::{DateTime, Utc};
use futures::TryStreamExt; // try_next()
use serde_json::Value;
use sqlx::postgres::types::{PgInterval, PgMoney, PgTimeTz};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::prelude::*;
use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
struct TypeTest {
    x_bigserial: i64,
    x_bigint: i64,
    x_integer: i32,
    x_smallint: i16,
    x_numeric: BigDecimal,
    x_double_precision: f64,
    x_real: f32,
    x_money: PgMoney,
    x_gender: Gender,
    x_boolean: bool,
    x_bit: BitVec,
    x_varbit: BitVec,
    x_character: String,
    x_varchar: String,
    x_text: String,
    x_bytea: Vec<u8>,
    x_uuid: Uuid,
    x_json: Value,
    x_jsonb: Value,
    x_date: NaiveDate,
    x_time: NaiveTime,
    x_timetz: PgTimeTz,
    x_timestamp: NaiveDateTime,
    x_timestamptz: DateTime<Utc>,
    x_interval: PgInterval,
    x_inet4: IpNetwork,
    x_cidr4: IpNetwork,
    x_inet6: IpNetwork,
    x_cidr6: IpNetwork,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
enum Gender {
    Male,
    Female,
    Other,
}

async fn new(conn_str: &str) -> Result<PgPool> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect(conn_str)
        .await?;

    Ok(conn)
}

async fn select(conn: &PgPool) -> Result<Vec<TypeTest>> {
    let mut rows = sqlx::query_as::<_, TypeTest>(
        r#"
SELECT x_bigserial
     , x_bigint
     , x_integer
     , x_smallint
     , x_numeric
     , x_double_precision
     , x_real
     , x_money
     , x_gender
     , x_boolean
     , x_bit
     , x_varbit
     , x_character
     , x_varchar
     , x_text
     , x_bytea
     , x_uuid
     , x_json
     , x_jsonb
     , x_date
     , x_time
     , x_timetz
     , x_timestamp
     , x_timestamptz
     , x_interval
     , x_inet4
     , x_cidr4
     , x_inet6
     , x_cidr6
  FROM sqlx.type_test
"#,
    )
    .fetch(conn);

    let mut ret = vec![];
    while let Some(row) = rows.try_next().await? {
        ret.push(row);
    }

    Ok(ret)
}

#[async_std::main]
async fn main() -> Result<()> {
    let conn = new("postgres://admin:admin@localhost:15432/sampledb").await?;

    let rows = select(&conn).await?;
    for row in rows {
        println!("{:#?}", row);
    }

    Ok(())
}
