# Setup

For each example in this tutorial, it will be useful to create yourself a separate work directory in which to put your code and compiled artifacts. This will help keep your files for each example organized.

In addition to a SQL client and SingleStore database, you'll need Docker or some other container runtime.  SingleStore's recommended workflow uses a special development container to ensure that you have all the dependencies needed to build a Wasm UDF or UDAF.  To help with this, we've created a handy [wrapper script](https://github.com/singlestore-labs/singlestore-wasm-toolkit/blob/main/scripts/dev-shell) that will start the shell.  You can download the script directly [here](https://raw.githubusercontent.com/singlestore-labs/singlestore-wasm-toolkit/refs/heads/main/scripts/dev-shell).

Once you've downloaded the script, ensure that it has executable permissions and is in your `PATH`.  Then, open a command prompt and run the following command.  The argument is the path to the intended root of your Wasm module's source code (it must exist first).  For example: `dev-shell /home/$USER/src/my-wasm-project`.

When the shell starts, you should see the following prompt:
```
[dev-shell]:~/src %
```

The `src` directory will have been mounted from path you passed to the `dev-shell` script.  It is *not* necessary to write the code for this tutorial inside the container's shell; you may use your preferred editing workflow for this.  However, please *do* be sure to run all suggested build and deployment commands inside the container's shell so that you have access to the necessary tools.

Next, let's pick an [example](Tutorial-Examples.md) to work through.

