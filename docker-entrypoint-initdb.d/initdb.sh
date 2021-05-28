set -e
psql -U admin sampledb <<EOSQL
CREATE SCHEMA sqlx;

CREATE TYPE gender AS ENUM ('male', 'female', 'other');

--
-- ref.) https://www.postgresql.org/docs/13/rangetypes.html
-- implement timerange on postgresql
--
CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS
'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE;

CREATE TYPE timerange AS RANGE (
    subtype = time,
    subtype_diff = time_subtype_diff
);


CREATE TABLE sqlx.type_test (
  x_bigserial              BIGSERIAL,
  x_bigint                 BIGINT, -- INT8
  x_newtype                BIGINT,
  x_integer                INT, -- INT4,
  x_smallint               SMALLINT, -- INT2
  x_numeric                NUMERIC(9,2),
  x_double_precision       DOUBLE PRECISION, -- FLOAT8,
  x_real                   REAL, -- FLOAT4,
  x_money                  MONEY,
  x_gender                 GENDER,
  x_weekday                INT, -- INT4
  x_boolean                BOOLEAN,
  x_bit                    bit(3),
  x_varbit                 varbit(20),
  x_character              CHARACTER(8),
  x_varchar                VARCHAR(20),
  x_text                   TEXT,
  x_bytea                  BYTEA,
  x_uuid                   UUID DEFAULT GEN_RANDOM_UUID(),
  x_json_typ               JSON,
  x_json_val               JSON,
  x_jsonb_typ              JSONB,
  x_jsonb_val              JSONB,
  x_date                   DATE DEFAULT CURRENT_DATE,
  x_time                   TIME DEFAULT CURRENT_TIME,
  x_timetz                 TIMETZ DEFAULT CURRENT_TIME,
  x_timestamp              TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  x_timestamptz            TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
  x_interval               INTERVAL,
  x_inet4                  INET,
  x_cidr4                  CIDR,
  x_inet6                  INET,
  x_cidr6                  CIDR,
  x_int4range              INT4RANGE,
  x_int8range              INT8RANGE,
  x_numrange               NUMRANGE,
  x_tsrange                TSRANGE,
  x_tstzrange              TSTZRANGE,
  x_daterange              DATERANGE,
  x_timerange              TIMERANGE, -- custom range


  PRIMARY KEY (x_bigserial)
);

-- SAMPLE DATA
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
  , x_json_typ
  , x_json_val
  , x_jsonb_typ
  , x_jsonb_val
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
  , x_timerange
  ) VALUES
  ( 6174
  , 123456789
  , 495
  , 42
  , 1234567.89
  , 1.23456789
  , 1.2345
  , 12.34
  , 'female'
  , 4 -- Thursday
  , 't'
  , b'101'
  , b'1010110'
  , 'Hello'
  , 'World!'
  , 'Long long ago, There are a boy...'
  , '🍣'
  , json_build_object('name', 'cutsea110', 'age', 50, 'height', 176.0, 'weight', 72.95, 'gender', 'Male', 'favorite', null)
  , json_build_object('name', 'cutsea110', 'age', 50, 'height', 176.0, 'weight', 72.95, 'favorite', 'swim')
  , array_to_json('{{1,2,3},{4,5,6},{7,8,9}}'::int[])
  , array_to_json('{{1,2,3},{4,5,6},{7,8,9}}'::float[])
  , make_interval(months := 14)
  , inet '192.168.0.0/24'
  , cidr '192.168.0.0/24'
  , inet '2001:db8:abcd:0012::0/64'
  , cidr '2001:db8:abcd:0012::0/64'
  , '[0,18)'
  , '[18,)'
  , '(0,42]'
  , '[2021-05-26 14:30, 2021-05-26 16:30)'
  , '[2021-05-26 00:00,)'
  , '[2021-01-01, 2021-12-31]'
  , '[13:00, 18:45)'
  );

EOSQL
