# Using `Cargo`

- Create project `cargo new`

- Build project `cargo build`

- Build and run in one step `cargo run`

- Build without producing a binary to check for errors using `cargo check`

- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.

- Finally, project ready for release `cargo build --release`. The output in *target/release*. This command, Cargo will optimizes, so that make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile
