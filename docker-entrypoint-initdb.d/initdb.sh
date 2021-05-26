set -e
psql -U admin sampledb <<EOSQL
CREATE TABLE type_test (
  x_bigserial              BIGSERIAL,
  x_bitint                 BIGINT,
  -- x_character              CHARACTER(8),
  -- x_varchar                VARCHAR(20),
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
  -- x_timestamptz            TIMESTAMPTZ,

  PRIMARY KEY (x_bigserial)
);
EOSQL
