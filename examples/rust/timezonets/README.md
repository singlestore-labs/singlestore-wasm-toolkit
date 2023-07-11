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

## Usage Example

## Building

### Compilation

To build this project, you will need to ensure that you have the
[WASI SDK](https://github.com/WebAssembly/wasi-sdk/releases) installed.  Please
set the environment variable `WASI_SDK_PATH` to its top-level directory.

If you change the `extension.wit` file, you will need to regenerate the ABI
wrappers.  To do this, make sure you have the wit-bindgen program installed.
Currently, SingleStoreDB only supports code generated using
[wit-bindgen v0.2.0](https://github.com/bytecodealliance/wit-bindgen/releases/tag/v0.2.0).

To compile:
```
make release
```

### Cleaning

To remove just the Wasm file:
```
make clean
```



