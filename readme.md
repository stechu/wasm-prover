Benchmark WASM Prover Perf
===========================

We implement a simple zkSNARK instance, pedersen commitment, to benchmark the wasm and native execution time.

## Dependencies:
* [Rust toolchain](https://www.rust-lang.org/tools/install)
* [npm](https://www.npmjs.com/get-npm)
* `wasm-pack` package:
    ```bash
    cargo install wasm-pack
    ```

## Run the benchmark
* WASM time:
    ```bash
    ./serve.sh
    ```
    You can view the result at `localhost:8080`.
* Native time:
    ```bash
    cargo bench
    ```
