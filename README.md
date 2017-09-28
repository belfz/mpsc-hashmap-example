# mpsc-hashmap-example

This is an example of using [multi-producer, single-consumer (MPSC)](https://doc.rust-lang.org/std/sync/mpsc/) in Rust for parallel computations and results aggregation.

The sole purpose of this example appplication is to count the number of occurences of each unique word in the sample text, by dividing it into smaller chunks and running calculations on multiple threads.

For details, follow the line-by-line comments in `src/main.rs`.

### How to...?

Run the project with `cargo run`.

For more realistic (optimised) results, first build the binary with `cargo build --release` and then run with `target/release/mpsc-hashmap`.
