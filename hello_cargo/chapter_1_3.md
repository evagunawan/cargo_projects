# Hello, Cargo
Cargo is a build system and package manager. It is commonly used in Rust because it will build your code, download dependencies, and build those dependencies. 

To check your cargo version
```
cargo --version
```

To create a new directory in cargo, use
```
cargo new <DIR>
```

This command will create 2 files and 1 directory like so:
```
.
├── Cargo.toml
└── src
    └── main.rs
```

It will also automatically initiate a github repo with a .gitignore file. If you run cargo new with an existing git repo, it will not override your current git files

```
[package] // Indicates that the following lines configure a package
name = "hello_cargo" // Name of the package
version = "0.1.0" // Version of the package
edition = "2021" // Edition of Rust to use

[dependencies] // Section to list any dependencies 
```

Packages of code are referred to as crates. 

The top level of the directory is meant for README, licenses, configs, and things not related to your code. Entering into the src directory is where source code lives.  

# Building and Running a cargo project
To begin to build use
```
cargo build
```

This will create an executable file in target/debug/hello_cargo rather than in your current directory. Since this project is still being developed, the binary is in the debug directory. 

To run the exe, use 
```
./target/debug/hello_cargo
```

To build and run all in one command, use:
```
cargo run
```

To not run your code but just check your code to make sure it can compile, you can use `cargo check`. This will save time to make sure your code is okay but not waste time creating the executable:
```
cargo check
```

# Building for release
To finalize the project release, use this to optimize the compilation. Running this makes your program run faster but your compilation is slower. Running `cargo build` will make your building faster than it would be in the released project. 
```
cargo build --release
```

