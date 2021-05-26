set -e
psql -U admin sampledb <<EOSQL
CREATE SCHEMA sqlx;

CREATE TYPE gender AS ENUM ('male', 'female', 'other');

CREATE TABLE sqlx.type_test (
  x_bigserial              BIGSERIAL,
  x_bigint                 BIGINT, -- INT8
  x_integer                INT, -- INT4,
  x_smallint               SMALLINT, -- INT2
  x_gender                 GENDER,
  x_character              CHARACTER(8),
  x_varchar                VARCHAR(20),
  x_text                   TEXT,
  x_uuid                   UUID DEFAULT GEN_RANDOM_UUID(),
  -- x_double_precision       FLOAT8,
  -- x_numeric                NUMERIC(9,2),
  -- x_real                   FLOAT4,
  x_date                   DATE DEFAULT CURRENT_DATE,
  x_time                   TIME DEFAULT CURRENT_TIME,
  -- x_timetz                 TIMETZ DEFAULT CURRENT_TIME,
  x_timestamp              TIMESTAMP DEFAULT NOW(),
  x_timestamptz            TIMESTAMPTZ DEFAULT NOW(),

  PRIMARY KEY (x_bigserial)
);

-- SAMPLE DATA
INSERT INTO sqlx.type_test
  ( x_bigint
  , x_integer
  , x_smallint
  , x_gender
  , x_character
  , x_varchar
  , x_text
  ) VALUES
  ( 6174
  , 495
  , 42
  , 'female'
  , 'Hello'
  , 'World!'
  , 'Long long ago, There are a boy...'
  );
EOSQL
