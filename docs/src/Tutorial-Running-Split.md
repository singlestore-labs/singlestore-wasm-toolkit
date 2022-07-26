# Running In the Database

Hooray, we can now run our Wasm function in the database!

When we imported our function, the database automatically converted the hyphens (`-`) in our function name to underscores (`_`).  So, `split-str` is now a UDF called `split_str`.

Using the following syntax, we can run our function as a TVF:
```sql
SELECT * FROM split_str('wasm_rocks_the_house', '_');
```

... which will produce the following output:
```console
+-------+-----+
| str   | idx |
+-------+-----+
| wasm  |   0 |
| rocks |   5 |
| the   |  11 |
| house |  15 |
+-------+-----+
94 rows in set (0.001 sec)
```

Awesome!

Let's [review what we've learned](Tutorial-WrapUp.md).

