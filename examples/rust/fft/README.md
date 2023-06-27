# Prerequisite
The easieast way is to install [dev-shell](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/scripts/dev-shell) and then run `dev-shell .` from this repository

# Functions in this module:
`st_process_forward(l: u8, nums: vector<Complex>)`: `l` is the size of our `Fft` (internally, this is used to define `plan_fft_forward` of `FftPlanner`, this length has to divide the length of `nums`. This will perform forward fourier transform. Same requirement for `st_process_inverse`, which perform inverse fourier transform.

Refer to [Rustfft documents](https://docs.rs/rustfft/latest/rustfft/struct.FftPlanner.html#method.plan_fft_forward) for more details.

# Compiling
    ```sh
    cargo build --target wasm32-wasi
    ```

# Testing
    ```sh
    cargo run --bin test
    ```

# Cleaning
    ```sh
    cargo clean
    ```

# Push to database
    After running `dev-shell`:
    ```
    ../bin/pushwasm tvf   \n
    -n st_process_forward \n
    --wasm target/wasm32-wasi/debug/fft.wasm     \n
    --wit fft.wit  --abi canonical --conn 'mysql://root@ip_address_of_your_singlestore_db/db_name'
    ```
# Usage in database
    ```
    SELECT * FROM (st_process_forward(1, [ROW(1.0, 2.5), ROW(2.0, 2.5)]));
    ```

