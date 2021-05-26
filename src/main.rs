use anyhow::Result;
use futures::TryStreamExt; // try_next()
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::prelude::*;

#[derive(Debug, sqlx::FromRow)]
struct TypeTest {
    x_bigserial: i64,
    x_bigint: i64,
    x_smallint: i16,
    x_gender: Gender,
    x_character: String,
    x_varchar: String,
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
     , x_smallint
     , x_gender
     , x_character
     , x_varchar
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
        println!("{:?}", row);
    }

    Ok(())
}
