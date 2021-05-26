set -e
psql -U admin sampledb <<EOSQL
CREATE SCHEMA sqlx;

CREATE TYPE gender AS ENUM ('male', 'female', 'other');

CREATE TABLE sqlx.type_test (
  x_bigserial              BIGSERIAL,
  x_bigint                 BIGINT, -- INT8
  x_integer                INT, -- INT4,
  x_smallint               SMALLINT, -- INT2
  x_numeric                NUMERIC(9,2),
  x_double_precision       DOUBLE PRECISION, -- FLOAT8,
  x_real                   REAL, -- FLOAT4,
  x_money                  MONEY,
  x_gender                 GENDER,
  x_boolean                BOOLEAN,
  x_bit                    bit(3),
  x_varbit                 varbit(20),
  x_character              CHARACTER(8),
  x_varchar                VARCHAR(20),
  x_text                   TEXT,
  x_bytea                  BYTEA,
  x_uuid                   UUID DEFAULT GEN_RANDOM_UUID(),
  x_json                   JSON,
  x_jsonb                  JSONB,
  x_date                   DATE DEFAULT CURRENT_DATE,
  x_time                   TIME DEFAULT CURRENT_TIME,
  x_timetz                 TIMETZ DEFAULT CURRENT_TIME,
  x_timestamp              TIMESTAMP DEFAULT NOW(),
  x_timestamptz            TIMESTAMPTZ DEFAULT NOW(),
  x_interval               INTERVAL,
  x_inet4                  INET,
  x_cidr4                  CIDR,
  x_inet6                  INET,
  x_cidr6                  CIDR,

  PRIMARY KEY (x_bigserial)
);

-- SAMPLE DATA
INSERT INTO sqlx.type_test
  ( x_bigint
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
  , x_json
  , x_jsonb
  , x_interval
  , x_inet4
  , x_cidr4
  , x_inet6
  , x_cidr6
  ) VALUES
  ( 6174
  , 495
  , 42
  , 1234567.89
  , 1.23456789
  , 1.2345
  , 12.34
  , 'female'
  , 't'
  , b'101'
  , b'1010110'
  , 'Hello'
  , 'World!'
  , 'Long long ago, There are a boy...'
  , '🍣'
  , json_object('{name, "cutsea110", age, 50, height, 176.0, weight, 72.95, sex, null, favorite, null}')
  , array_to_json('{{1,2,3},{4,5,6},{7,8,9}}'::int[])
  , make_interval(months := 14)
  , inet '192.168.0.0/24'
  , cidr '192.168.0.0/24'
  , inet '2001:db8:abcd:0012::0/64'
  , cidr '2001:db8:abcd:0012::0/64'
  );
EOSQL
