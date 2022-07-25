# Running In the Database

Hooray, we can now run our Wasm function in the database!

When we imported our function, the database automatically converted the hyphen
s (`-`) in our function name to underscores (`_`).  So, `power-of` is now a UDF called `power_of`.

Using the following syntax, we can run it as a UDF:

```sql
SELECT power_of(2, 8);
```

... which should return a single-column row with the value `256`.

Neat!

Let's [review what we've learned](Tutorial-WrapUp.md).

