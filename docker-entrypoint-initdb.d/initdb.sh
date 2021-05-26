set -e
psql -U admin sampledb <<EOSQL
CREATE SCHEMA sqlx;

CREATE TYPE gender AS ENUM ('male', 'female', 'other');

CREATE TABLE sqlx.type_test (
  x_bigserial              BIGSERIAL,
  x_bigint                 BIGINT,
  x_smallint               SMALLINT,
  x_gender                 GENDER,
  x_character              CHARACTER(8),
  x_varchar                VARCHAR(20),
  -- x_date                   DATE,
  -- x_double_precision       FLOAT8,
  -- x_integer                INT4,
  -- x_numeric                NUMERIC(9,2),
  -- x_real                   FLOAT4,
  -- x_smallint               INT2,
  -- x_text                   TEXT,
  -- x_time                   TIME,
  -- x_timetz                 TIMETZ,
  -- x_timestamp              TIMESTAMP,
  x_timestamptz            TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  PRIMARY KEY (x_bigserial)
);

-- SAMPLE DATA
INSERT INTO sqlx.type_test
  ( x_bigint
  , x_smallint
  , x_gender
  , x_character
  , x_varchar
  ) VALUES
  ( 42
  , 99
  , 'female'
  , 'Hello'
  , 'World!'
  );
EOSQL
