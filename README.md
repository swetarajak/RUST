# RUST
-- Complete installation from thr rust official site 
[Go to the Installation](https://www.rust-lang.org/tools/install)

<hr>

## Run a Rust program (Hello, World)
 - Make a directory
 - Make a folder(hello_world) -> Naming convention : Name files using underscores
 - Make a file (main.rs) -> extension of rust : rs
 
 ### Compile and run the file 
 ```
    # Compile the program 
    rustc main.rs

    # Run the compiled program
    .\main.rs (Windows) | ./main.rs (Linux or macOS) 
```
## Creating a project with Cargo
```
    # Create a new directory
    cargo new hello_cargo
    cd hello_cargo

    # Build your project
    cargo build

    #Run the executable
    ./target/debug/hello_cargo  or 
    .\target\debug\hello_cargo.exe on Windows

    # Build and run the project using single command
    cargo run
```
      