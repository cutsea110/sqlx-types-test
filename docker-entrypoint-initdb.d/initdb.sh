set -e
psql -U admin sampledb <<EOSQL
CREATE SCHEMA sqlx;

CREATE TYPE gender AS ENUM ('male', 'female', 'other');

CREATE TABLE sqlx.type_test (
  x_bigserial              BIGSERIAL,
  x_bigint                 BIGINT, -- INT8
  x_integer                INT, -- INT4,
  x_smallint               SMALLINT, -- INT2
  x_double_precision       DOUBLE PRECISION, -- FLOAT8,
  x_real                   REAL, -- FLOAT4,
  x_money                  MONEY,
  x_gender                 GENDER,
  x_character              CHARACTER(8),
  x_varchar                VARCHAR(20),
  x_text                   TEXT,
  x_bytea                  BYTEA,
  x_uuid                   UUID DEFAULT GEN_RANDOM_UUID(),
  -- x_numeric                NUMERIC(9,2),
  x_date                   DATE DEFAULT CURRENT_DATE,
  x_time                   TIME DEFAULT CURRENT_TIME,
  x_timetz                 TIMETZ DEFAULT CURRENT_TIME,
  x_timestamp              TIMESTAMP DEFAULT NOW(),
  x_timestamptz            TIMESTAMPTZ DEFAULT NOW(),

  PRIMARY KEY (x_bigserial)
);

-- SAMPLE DATA
INSERT INTO sqlx.type_test
  ( x_bigint
  , x_integer
  , x_smallint
  , x_double_precision
  , x_real
  , x_money
  , x_gender
  , x_character
  , x_varchar
  , x_text
  , x_bytea
  ) VALUES
  ( 6174
  , 495
  , 42
  , 1.23456789
  , 1.2345
  , 12.34
  , 'female'
  , 'Hello'
  , 'World!'
  , 'Long long ago, There are a boy...'
  , '🍣'
  );
EOSQL
