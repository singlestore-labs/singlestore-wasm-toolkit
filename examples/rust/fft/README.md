# Fast Fourier Transform in SingleStoreDB

## Introduction

[Fast Fourier transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform) is an algorithm that computes the discrete Fourier transform (DFT) of a sequence, or its inverse (IDFT).
Fourier analysis converts a signal from its original domain (often time or space) to a representation in the frequency domain and vice versa. The DFT is obtained by decomposing a sequence of values into components of different frequencies.
Fast Fourier transforms are widely used for applications in engineering, music, science, and mathematics. The FFT is used in digital recording, sampling, additive synthesis and pitch correction software.

This library uses [rustfft](https://docs.rs/rustfft/latest/rustfft/) for algorithm implementation. This is a high-performance, SIMD-accelerated FFT library, compute FFTs of any size, in O(nlogn) time. Support for hardware acceleration (Avx, Neon, Sse) from rustfft is not ported to this module.

## Contents
This library provides the following database objects.

### `st_planner_forward(len: u8, buffer: [Complex<f64>])`
This is a TVF that will create a a new FFT algorthim instance for computing forward FFT of size `len`. Divides the `buffer` into chunks of size `len`, and computes FFT forward on each chunk.
This method will panic (on Rust side) if:
```
buffer.len() % len > 0
buffer.len() < len
```

### `st_planner_inverse(len: u8, buffer: [Complex<f64>])`
This is a TVF that will create a a new FFT algorthim instance for computing inverse FFT of size `len`. Divides the `buffer` into chunks of size `len`, and computes FFT inverse on each chunk.
This method will panic (on Rust side) if:
```
buffer.len() % len > 0
buffer.len() < len
```

## Building
The Wasm module can be built using the following commands.  The build requires Rust with the WASI extension.
```bash
# Install the WASI cargo extension.
cargo install cargo-wasi

# Compile the Wasm module.
cargo wasi build --release
```
The binary will be placed in `target/wasm32-wasi/release/fft.wasm`.

## Deployment to SingleStoreDB

To install these functions using the MySQL client, use the following commands.  This command assumes you have built the Wasm module and your current directory is the root of this Git repo.  Replace `$DBUSER`, `$DBHOST`, `$DBPORT`, and `$DBNAME` with, respectively, your database username, hostname, port, and the name of the database where you want to deploy the functions.
```bash
cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE FUNCTION st_process_forward RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/fft.wasm" WITH WIT FROM LOCAL INFILE "fft.wit";
CREATE FUNCTION st_process_inverse RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/fft.wasm" WITH WIT FROM LOCAL INFILE "fft.wit";
```

Alternatively, you can install these functions using [pushwasm](https://github.com/singlestore-labs/pushwasm) with the following command lines.  As above, be sure to substitute the environment variables with values of your own.
```bash
pushwasm tvf --force --prompt --name st_process_forward \
    --wasm target/wasm32-wasi/release/fft.wasm \
    --wit fft.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
pushwasm tvf --force --prompt --name st_process_inverse \
    --wasm target/wasm32-wasi/release/fft.wasm \
    --wit fft.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
```

## Usage
The following is a simple example that creates two tables with a columns of strings.  The first table's column is used to generate a Bloom Filter, which we store in a User Defined Variable.  We then run the Bloom Filter on the strings in the second table.
The following is a simple example that performs forward FFT on a vector `buffer` of two complex numbers `{"re": 1.0, "im": 2.5}` and `{"re": 2.0, "im": 2.5}`. This will divides the vector `buffer` into chunks of size `1` and computes a FFT on each chunk.
```sql
SELECT * FROM (st_process_forward(1, [ROW(1.0, 2.5), ROW(2.0, 2.5)]));
```

This should produce the following output:
```console
+----+-----+
| re | im  |
+----+-----+
|  1 | 2.5 |
|  2 | 2.5 |
+----+-----+
2 rows in set (0.004 sec)
```

## Additional Information

To learn about the process of developing a Wasm UDF or TVF in more detail, please have a look at our [tutorial](https://singlestore-labs.github.io/singlestore-wasm-toolkit/html/Tutorial-Overview.html).

The SingleStoreDB Wasm UDF/TVF documentation is [here](https://docs.singlestore.com/managed-service/en/reference/code-engine---powered-by-wasm.html).

## Resources

* [Fast Fourier Transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform)
* [Rust FFT library](https://docs.rs/rustfft/latest/rustfft/)
* [Documentation](https://docs.singlestore.com)
* [Twitter](https://twitter.com/SingleStoreDevs)
* [SingleStore forums](https://www.singlestore.com/forum)
* [SingleStoreDB extension template for C++](https://github.com/singlestore-labs/singlestoredb-extension-cpp-template)
