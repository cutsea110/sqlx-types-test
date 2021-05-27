use anyhow::Result;
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono::{DateTime, Utc};
use futures::TryStreamExt; // try_next()
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::postgres::types::{PgInterval, PgMoney, PgRange, PgTimeTz};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::{ipnetwork::IpNetwork, BigDecimal, BitVec, Json, Uuid};

// Serialize, Deserialize は Json で扱うので必要になる
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct User {
    name: String,
    age: i16,
    height: f32,
    weight: f32,
    gender: Option<Gender>,
    favorite: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
struct TypeTestId(i64);

#[derive(Debug, sqlx::FromRow)]
struct TypeTest {
    x_bigserial: i64,
    x_bigint: i64,
    x_newtype: TypeTestId, // New Type
    x_integer: i32,
    x_smallint: i16,
    x_numeric: BigDecimal,
    x_double_precision: f64,
    x_real: f32,
    x_money: PgMoney,
    x_gender: Gender,   // Strong Enum
    x_weekday: WeekDay, // Weak Enum
    x_boolean: bool,
    x_bit: BitVec,
    x_varbit: BitVec,
    x_character: String,
    x_varchar: String,
    x_text: String,
    x_bytea: Vec<u8>,
    x_uuid: Uuid,
    x_json_typ: Json<User>,
    x_json_val: JsonValue,
    x_jsonb_typ: Json<Vec<Vec<i32>>>,
    x_jsonb_val: JsonValue,
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
    x_int4range: PgRange<i32>,
    x_int8range: PgRange<i64>,
    x_numrange: PgRange<BigDecimal>,
    x_tsrange: PgRange<NaiveDateTime>,
    x_tstzrange: PgRange<DateTime<Utc>>,
    x_daterange: PgRange<NaiveDate>,
}

// Serialize, Deserialize は Json でも使われているので必要
// User が Json になっていて、この型を含んでいるため
#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "gender")] // for postgres only
#[sqlx(rename_all = "lowercase")]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, sqlx::Type)]
#[repr(i32)]
enum WeekDay {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
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
     , x_newtype
     , x_integer
     , x_smallint
     , x_numeric
     , x_double_precision
     , x_real
     , x_money
     , x_gender
     , x_weekday
     , x_boolean
     , x_bit
     , x_varbit
     , x_character
     , x_varchar
     , x_text
     , x_bytea
     , x_uuid
     , x_json_typ
     , x_json_val
     , x_jsonb_typ
     , x_jsonb_val
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
     , x_int4range
     , x_int8range
     , x_numrange
     , x_tsrange
     , x_tstzrange
     , x_daterange
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

async fn insert(conn: &PgPool, v: TypeTest) -> Result<TypeTest> {
    let row = sqlx::query_as::<_, TypeTest>(
        r#"
INSERT INTO sqlx.type_test
  ( x_bigint
  , x_newtype
  , x_integer
  , x_smallint
  , x_numeric
  , x_double_precision
  , x_real
  , x_money
  , x_gender
  , x_weekday
  , x_boolean
  , x_bit
  , x_varbit
  , x_character
  , x_varchar
  , x_text
  , x_bytea
  , x_uuid
  , x_json_typ
  , x_json_val
  , x_jsonb_typ
  , x_jsonb_val
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
  , x_int4range
  , x_int8range
  , x_numrange
  , x_tsrange
  , x_tstzrange
  , x_daterange
  ) VALUES
  ( $1
  , $2
  , $3
  , $4
  , $5
  , $6
  , $7
  , $8
  , $9
  , $10
  , $11
  , $12
  , $13
  , $14
  , $15
  , $16
  , $17
  , $18
  , $19
  , $20
  , $21
  , $22
  , $23
  , $24
  , $25
  , $26
  , $27
  , $28
  , $29
  , $30
  , $31
  , $32
  , $33
  , $34
  , $35
  , $36
  , $37
  , $38
  )
RETURNING *
"#,
    )
    .bind(v.x_bigint)
    .bind(v.x_newtype)
    .bind(v.x_integer)
    .bind(v.x_smallint)
    .bind(v.x_numeric)
    .bind(v.x_double_precision)
    .bind(v.x_real)
    .bind(v.x_money)
    .bind(v.x_gender)
    .bind(v.x_weekday)
    .bind(v.x_boolean)
    .bind(v.x_bit)
    .bind(v.x_varbit)
    .bind(v.x_character)
    .bind(v.x_varchar)
    .bind(v.x_text)
    .bind(v.x_bytea)
    .bind(v.x_uuid)
    .bind(v.x_json_typ)
    .bind(v.x_json_val)
    .bind(v.x_jsonb_typ)
    .bind(v.x_jsonb_val)
    .bind(v.x_date)
    .bind(v.x_time)
    .bind(v.x_timetz)
    .bind(v.x_timestamp)
    .bind(v.x_timestamptz)
    .bind(v.x_interval)
    .bind(v.x_inet4)
    .bind(v.x_cidr4)
    .bind(v.x_inet6)
    .bind(v.x_cidr6)
    .bind(v.x_int4range)
    .bind(v.x_int8range)
    .bind(v.x_numrange)
    .bind(v.x_tsrange)
    .bind(v.x_tstzrange)
    .bind(v.x_daterange)
    .fetch_one(conn)
    .await?;

    Ok(row)
}

async fn delete_others(conn: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
DELETE FROM sqlx.type_test
 WHERE x_bigserial <> (SELECT min(x_bigserial) FROM sqlx.type_test)
"#,
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn count(conn: &PgPool) -> Result<i64> {
    let c: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlx.type_test")
        .fetch_one(conn)
        .await?;

    Ok(c.0)
}

#[async_std::main]
async fn main() -> Result<()> {
    let conn = new("postgres://admin:admin@localhost:15432/sampledb").await?;

    let rows = select(&conn).await?;
    for row in rows {
        println!("{:#?}", row);
        let row = insert(&conn, row).await?;
        println!("{:#?}", row);
    }

    delete_others(&conn).await?;

    let c = count(&conn).await?;
    println!("COUNT: {:?}", c);

    Ok(())
}
