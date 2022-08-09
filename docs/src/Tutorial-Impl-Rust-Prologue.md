## Initialize Your Source Tree                                                   

To setup our project, we'll need to do a few things:

1. Ensure that you are in a new directory.  It can be called anything you want.

1. In the work directory, run the following to set up a skeltal Rust source tree:
   ```
   cargo init --vcs none --lib
   ```
                                                                                 
1. Now, create a special configuration file to tell utilities like the *rust-analyzer* that the default build target of this project is Wasm.  Run this command:
   ```bash                                                                          
   mkdir .cargo && echo -e "[build]\ntarget = wasm32-wasi\n" > .cargo/config.toml   
   ```                                                                              
