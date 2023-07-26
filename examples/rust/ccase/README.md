
# Functionalities for timestamp with timezone

## Introduction

This library aims to support converting string case

## Contents
Currently, this library provides the following UDFs:

### `change_case (s: string, case-type: int)`
Change case of a given string, `case_type` is a number from 1-19 corresponding to the following:
   1. Alternating
   2. Camel
   3. Cobol
   4. Flat
   5. Kebab
   6. Lower
   7. Pascal
   8. PseudoRandom
   9. Random
   10. ScreamingSnake
   11. Snake
   12. Title
   13. Toggle
   14. Train
   15. Upper
   16. UpperCamel
   17. UpperFlat
   18. UpperKebab
   19. UpperSnake
Any number outside of this range will return the original string

#### Return type
String

## Deployment to SingleStoreDB

To install these functions using the MySQL client, use the following commands.  This command assumes you have built the Wasm module and your current directory is the root of this Git repo.  Replace `$DBUSER`, `$DBHOST`, `$DBPORT`, and `$DBNAME` with, respectively, your database username, hostname, port, and the name of the database where you want to deploy the functions.
```bash
cat <<EOF | mysql -u $DBUSER -h $DBHOST -P $DBPORT -D $DBNAME -p
CREATE FUNCTION change_case RETURNS TABLE AS WASM FROM LOCAL INFILE "target/wasm32-wasi/release/ccase.wasm" WITH WIT FROM LOCAL INFILE "ccase.wit";
```

Alternatively, you can install these functions using [pushwasm](https://github.com/singlestore-labs/pushwasm) with the following command lines.  As above, be sure to substitute the environment variables with values of your own.
```bash
pushwasm udf --force --prompt --name change_case \
    --wasm target/wasm32-wasi/release/ccase.wasm \
    --wit ccase.wit \
    --abi canonical \
    --conn "mysql://$DBUSER@$DBHOST:$DBPORT/$DBNAME"
```

## Clean
```
make clean
```

## Usage
```sql
SELECT change_case("My variable NAME", 1);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 1) |
+------------------------------------+
| mY vArIaBlE nAmE                   |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 2);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 2) |
+------------------------------------+
| myVariableName                     |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 3);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 3) |
+------------------------------------+
| MY-VARIABLE-NAME                   |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 4);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 4) |
+------------------------------------+
| myvariablename                     |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 5);
```

Output:
```
------------------------------------+
| change_case("My variable NAME", 5) |
+------------------------------------+
| my-variable-name                   |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 6);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 6) |
+------------------------------------+
| my variable name                   |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 8);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 8) |
+------------------------------------+
| MY variaBlE naME                   |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 9);
```

Output:
```
+------------------------------------+
| change_case("My variable NAME", 9) |
+------------------------------------+
| My VAriABlE NAME                   |
+------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 10);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 10) |
+-------------------------------------+
| MY_VARIABLE_NAME                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 11);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 11) |
+-------------------------------------+
| my_variable_name                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 12);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 12) |
+-------------------------------------+
| My Variable Name                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 13);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 13) |
+-------------------------------------+
| mY vARIABLE nAME                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 14);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 14) |
+-------------------------------------+
| My-Variable-Name                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 15);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 15) |
+-------------------------------------+
| MY VARIABLE NAME                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 16);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 16) |
+-------------------------------------+
| MyVariableName                      |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 17);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 17) |
+-------------------------------------+
| MYVARIABLENAME                      |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 18);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 18) |
+-------------------------------------+
| MY-VARIABLE-NAME                    |
+-------------------------------------+
```

```sql
SELECT change_case("My variable NAME", 19);
```

Output:
```
+-------------------------------------+
| change_case("My variable NAME", 19) |
+-------------------------------------+
| MY_VARIABLE_NAME                    |
+-------------------------------------+
```

## Acknowledgement
Rust [convert_case](https://docs.rs/convert_case/latest/convert_case/enum.Case.html#variant.Snake) library
