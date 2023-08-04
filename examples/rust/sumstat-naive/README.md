
```sql
cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE OR REPLACE AGGREGATE summary_statistics(double NOT NULL)
RETURNS LONGBLOB NOT NULL
WITH STATE HANDLE
AS WASM FROM BASE64 '`base64 -w 0 target/wasm32-wasi/release/extension.wasm`'
WITH WIT FROM BASE64 '`base64 -w 0 extension.wit`'
INITIALIZE WITH sumstat_naive_handle_init
ITERATE WITH sumstat_naive_handle_iter
MERGE WITH sumstat_naive_handle_merge
TERMINATE WITH sumstat_naive_handle_serialize
SERIALIZE WITH sumstat_naive_handle_serialize
DESERIALIZE WITH sumstat_naive_handle_deserialize
EOF

cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE OR REPLACE FUNCTION sumstat_naive_t_test_one
AS WASM FROM BASE64 '`base64 -w 0 target/wasm32-wasi/release/extension.wasm`'
WITH WIT FROM BASE64 '`base64 -w 0 extension.wit`'
EOF
```

Alternatively, you can install these functions using [pushwasm](https://github.com/singlestore-labs/pushwasm) with the following command lines. As above, be sure to substitute the environment variables with values of your own.

```sh
pushwasm udaf --force --prompt --name summary_statistics  \
    --wasm target/wasm32-wasi/release/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME" \
    --abi canonical \
    --type 'LONGBLOB not null' \
    --arg 'double not null' \
    --state HANDLE \
    --init sumstat_init_handle\
    --iter sumstat_update_handle \
    --merge sumstat_merge_handle \
    --terminate sumstat_serialize_handle \
    --serialize sumstat_serialize_handle \
    --deserialize sumstat_deserialize_handle

pushwasm udf --force --prompt --name sumstat_summary \
    --wasm target/wasm32-wasi/debug/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"

pushwasm udf --force --prompt --name sumstat_t_test_one \
    --wasm target/wasm32-wasi/debug/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"

pushwasm udf --force --prompt --name sumstat_t_test_paired \
    --wasm target/wasm32-wasi/debug/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"

pushwasm udf --force --prompt --name sumstat_t_test_indep \
    --wasm target/wasm32-wasi/debug/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"

pushwasm udf --force --prompt --name sumstat_t_test_indepu \
    --wasm target/wasm32-wasi/debug/extension.wasm \
    --wit extension.wit \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
```

