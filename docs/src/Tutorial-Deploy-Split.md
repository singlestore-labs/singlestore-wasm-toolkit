<!-- GENERATED FILE; DO NOT CHANGE! -->

# Deploying the `split-str` Example
Now that you've compiled and tested your Wasm function, it is ready for deployment into the database.  This can be done in multiple ways.  One way is to use the `pushwasm` tool, provided in the development container.  Alternatively, you can "pull" the Wasm module into the database by first uploading it to cloud storage (SingleStoreDB supports pulling Wasm modules from multiple cloud providers -- GCS, Azure, and S3).  We'll discuss both techniques.

Before we start, ensure that a destination database is available.  To do this, using your favorite SQL client, create a new database called `wasm_tutorial`.  For example, you might use the following statements:

```sql
CREATE DATABASE wasm_tutorial;
USE wasm_tutorial;
```


- To deploy the `split-str` example using the `pushwasm` tool, please see [here](Tutorial-Deploy-Split-Push.md).
- To deploy the `split-str` example from cloud storage, please see [here](Tutorial-Deploy-Split-Cloud.md).

