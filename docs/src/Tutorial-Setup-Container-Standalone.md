# Standalone Container Setup

This container provides a standalone shell environment configured with a variety of Wasm development tools.  It will mount the directory of your choosing.

1. Ensure that the *wasm-toolkit* repository has been cloned.

1. Create a subdirectory for the code in this tutorial.  This can be anywhere.  For the purposes of this tutorial, we'll assume it is in `/home/$USER/wasm-tutorial`.

1. At your command prompt, change to the root directory of the *wasm-toolkit* repository and type `scripts/dev-shell /home/$USER/wasm-tutorial`.  Ensure that the argument reflects the actual path of the directory you created in the above step.

1. You should now see the following prompt:
```
[s2-dev-shell]:~/src %
```

The `src` directory has been mounted from `/home/$USER/wasm-tutorial` (or whatever alternative directory you specified in step 3).  It is *not* necessary to write the code for this tutorial inside the container's shell; you may use your preferred editing workflow for this.  However, please *do* be sure to run all suggested build and deployment commands inside the container's shell so that you have access to the necessary tools.

Next, let's pick an [example](Tutorial-Examples.md) to work through.

