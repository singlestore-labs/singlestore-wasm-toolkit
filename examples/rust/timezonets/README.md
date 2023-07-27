# Functionalities for timestamp with timezone

## Introduction

This library aims to support basic operations on timestamp with timezone.

## Contents
Currently, this library provides the following UDFs:

### `unix_timestamp_fmt (result_unit, ts, fmt)`
Returns the given timestamp with a specified format into unixtime.

#### Args
* result_unit: millisecond, microsecond or nanosecond
* ts: string of the timestamp
* fmt: string of the timestamp format (strftime standard)

#### Return type
Integer


### `timestampadd_fmt (interval_unit, interval, ts, fmt, res_fmt)`
Similar to [TIMESTAMPADD](https://docs.singlestore.com/managed-service/en/reference/sql-reference/date-and-time-functions/timestampadd.html). Adds the given interval of time to a timestamp with a specified format

#### Args
* interval_unit: month | week | day | hour | minute | second | millisecond | nanosecond
* interval : integer
* ts: string of the timestamp
* fmt: string of the timestamp format (strftime standard)
* res_fmt: string of the result timestamp format (strftime standard)

#### Return type
Timestamp string


### `timestampdiff_fmt (result_unit, ts1, fmt1, ts2, fmt2)`
Similar to [TIMESTAMPDIFF](https://docs.singlestore.com/managed-service/en/reference/sql-reference/date-and-time-functions/timestampdiff.html). Find between 2 timestamps with specified formats the time difference in the specified unit.

#### Args
* result_unit: year | week | day | hour | minute | second | millisecond | nanosecond
* ts1, ts2: string of the timestamp
* fmt1, fmt2: string of the timestamp format (strftime standard)

#### Return type
Integer

### `extract_fmt (extract_unit, ts, fmt)`
Similar to [EXTRACT](https://docs.singlestore.com/managed-service/en/reference/sql-reference/date-and-time-functions/extract.html).Extracts specified components from a given timestamp

#### Args
* extract_unit: year | month | weekday | day | hour | minute | millisecond | microsecond | nanosecond
* ts: string of the timestamp
* fmt: string of the timestamp format (strftime standard)

#### Return type
Integer

### `convert_tz_fmt (ts, fmt, tz)`
Similar to [CONVERT_TZ](https://docs.singlestore.com/managed-service/en/reference/sql-reference/date-and-time-functions/convert_tz.html). Converts a given timestamp with a specified format to another timezone (a timestamp with no timzezone declaration will assume UTC)

#### Args
* ts: string of the timestamp
* fmt: string of the timestamp format (strftime standard)
* tz: string of the timezone name

#### Return type
Timestamp string

## Building
```
# Install the WASI cargo extension.
cargo install cargo-wasi

# Compile the Wasm module.
cargo wasi build --release
```

## Deployment to SingleStoreDB

To install these functions using the MySQL client, use the following commands.  This command assumes you have built the Wasm module and your current directory is the root of this Git repo.  Replace `$DBUSER`, `$DBHOST`, `$DBPORT`, and `$DBNAME` with, respectively, your database username, hostname, port, and the name of the database where you want to deploy the functions.
```bash
cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE FUNCTION unix_timestamp_fmt RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/tsz.wasm" WITH WIT FROM LOCAL INFILE "tsz.wit";
CREATE FUNCTION timestampadd_fmt RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/tsz.wasm" WITH WIT FROM LOCAL INFILE "tsz.wit";
CREATE FUNCTION timestampdiff_fmt RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/tsz.wasm" WITH WIT FROM LOCAL INFILE "tsz.wit";
CREATE FUNCTION convert_to_utc_fmt RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/tsz.wasm" WITH WIT FROM LOCAL INFILE "tsz.wit";
CREATE FUNCTION extract_fmt RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/tsz.wasm" WITH WIT FROM LOCAL INFILE "tsz.wit";
CREATE FUNCTION convert_tz_fmt RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/tsz.wasm" WITH WIT FROM LOCAL INFILE "tsz.wit";
```

Alternatively, you can install these functions using [pushwasm](https://github.com/singlestore-labs/pushwasm) with the following command lines.  As above, be sure to substitute the environment variables with values of your own.
```bash
pushwasm udf --force --prompt --name unix_timestamp_fmt \
    --wasm target/wasm32-wasi/release/tsz.wasm \
    --wit tsz.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm udf --force --prompt --name timestampadd_fmt \
    --wasm target/wasm32-wasi/release/tsz.wasm \
    --wit tsz.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm udf --force --prompt --name timestampdiff_fmt \
    --wasm target/wasm32-wasi/release/tsz.wasm \
    --wit tsz.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm udf --force --prompt --name convert_to_utc_fmt \
    --wasm target/wasm32-wasi/release/tsz.wasm \
    --wit tsz.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm udf --force --prompt --name extract_fmt \
    --wasm target/wasm32-wasi/release/tsz.wasm \
    --wit tsz.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm udf --force --prompt --name convert_tz_fmt \
    --wasm target/wasm32-wasi/release/tsz.wasm \
    --wit tsz.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
```

## Clean
```
make clean
```

## Usage
```sql
  SELECT unix_timestamp_fmt("nanosecond", "2009-02-13 20:31:30-03:00", "%Y-%m-%d
 %H:%M:%S%z");
```

Output:
```
+----------------------------------------------------------------------------------------+
| (unix_timestamp_fmt("nanosecond", "2009-02-13 20:31:30-03:00", "%Y-%m-%d %H:%M:%S%z")) |
+----------------------------------------------------------------------------------------+
|                                                                    1234567890000000000 |
+----------------------------------------------------------------------------------------+
```


```sql
SELECT convert_tz_fmt("2014-04-18 12:00:00+07:00", "%Y-%m-%d %H:%M:%S%z", "EST", "");
```

Output
```
+-------------------------------------------------------------------------------+
| convert_tz_fmt("2014-04-18 12:00:00+07:00", "%Y-%m-%d %H:%M:%S%z", "EST", "") |
+-------------------------------------------------------------------------------+
| 2014-04-18 00:00:00-0500                                                      |
+-------------------------------------------------------------------------------+
```

Note: `%Z` can only be used for formatting
```sql
SELECT convert_tz_fmt("2014-04-18 12:00:00+07:00", "%Y-%m-%d %H:%M:%S%z", "EST", "%Y-%m-%d %H-%M-%S %Z");
```

Output:
```
+---------------------------------------------------------------------------------------------------+
| convert_tz_fmt("2014-04-18 12:00:00+07:00", "%Y-%m-%d %H:%M:%S%z", "EST", "%Y-%m-%d %H-%M-%S %Z") |
+---------------------------------------------------------------------------------------------------+
| 2014-04-18 00-00-00 EST                                                                           |
+---------------------------------------------------------------------------------------------------+
```

```sql
SELECT timestampadd_fmt("week", 2, "2023-07-18 12:00:00-0500", "%Y-%m-%d %H:%M:%S%z", "%Y-%m-%
d %H:%M:%S%.6f%z");
```

Output:
```
+-----------------------------------------------------------------------------------------------------------+
| timestampadd_fmt("week", 2, "2023-07-18 12:00:00-0500", "%Y-%m-%d %H:%M:%S%z", "%Y-%m-%d %H:%M:%S%.6f%z") |
+-----------------------------------------------------------------------------------------------------------+
| 2023-08-01 12:00:00.000000-0500                                                                           |
+-----------------------------------------------------------------------------------------------------------+
```

```sql
SELECT timestampdiff_fmt("second", "2009-02-14 06:17:01+0500", "%Y-%m-%d %H:%M:%S%z", "2009-0
2-13 20:31:30-0300", "%Y-%m-%d %H:%M:%S%z");
```

Output:
```
+-----------------------------------------------------------------------------------------------------------------------------------+
| timestampdiff_fmt("second", "2009-02-14 06:17:01+0500", "%Y-%m-%d %H:%M:%S%z", "2009-02-13 20:31:30-0300", "%Y-%m-%d %H:%M:%S%z") |
+-----------------------------------------------------------------------------------------------------------------------------------+
|                                                                                                                             -6331 |
+-----------------------------------------------------------------------------------------------------------------------------------+
```

```sql
SELECT extract_fmt("nano",  "2019-03-25 10:15:21.000423986", "%Y-%m-%d %H:%M:%S%.9f");
```

Output:
```
+--------------------------------------------------------------------------------+
| extract_fmt("nano",  "2019-03-25 10:15:21.000423986", "%Y-%m-%d %H:%M:%S%.9f") |
+--------------------------------------------------------------------------------+
|                                                                         423986 |
+--------------------------------------------------------------------------------+
```

```sql
SELECT convert_to_utc_fmt("2014-04-18T12:00:00+05:00", "%+");
```

Output:
```
+-------------------------------------------------------+
| convert_to_utc_fmt("2014-04-18T12:00:00+05:00", "%+") |
+-------------------------------------------------------+
| 2014-04-18 07:00:00                                   |
+-------------------------------------------------------+
```

## Acknowledgement
Rust [chrono](https://docs.rs/chrono/latest/chrono) and [chrono-tz](https://github.com/chronotope/chrono-tz) library
