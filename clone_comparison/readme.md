# Performance Comparison: Rust `String` Clone vs. `Rc<String>` Clone

## 1. Introduction

This study aims to compare and analyze the performance of two primary methods for copying string (`String`) data in the Rust programming language: direct cloning using `String::clone()` and indirect cloning using `Rc<String>`. Rust ensures memory safety through powerful concepts such as Ownership, Borrowing, and Lifetimes, but sometimes requires data to be copied to maintain this safety. String data, in particular, is frequently used in programs, and copying operations can significantly impact performance. Therefore, considering performance is crucial when choosing a string cloning method.

This research seeks to clearly present the advantages and disadvantages of each method by measuring and comparing the execution time and memory usage of direct `String` cloning and cloning through shared ownership using `Rc<String>`.

## 2. Background

There are primarily two methods for copying string data in Rust:

*   **`String::clone()` (Direct Clone):** The `String::clone()` method performs a **deep copy** of the string data. It allocates new memory space with the same content as the original string data and copies the data. This method ensures that the original data and the copy are independent, so modifying the copy does not affect the original data. However, the time and memory usage required for copying increase with the size of the data.

*   **`Rc<String>::clone()` (Indirect Clone):** `Rc` is a reference-counting smart pointer that provides shared ownership of data. Cloning an `Rc<String>` creates a new `Rc` pointer that points to the **same heap memory address** as the original `Rc` pointer. The data itself is not copied; only the reference count is incremented. This method can reduce data copying costs, but the data accessed through `Rc<String>` is immutable and cannot be modified. Other methods must be used if the data needs to be modified.

## 3. Experimental Design

This study conducted the following experiments to compare and analyze the performance of `String` cloning and `Rc<String>` cloning:

*   **Data:** 1MB of string data (`String`)
*   **Comparison Targets:**
    *   `String::clone()`: Clone the `String` data N times directly.
    *   `Rc<String>::clone()`: Wrap the `String` data with `Rc` and clone the `Rc` pointer N times.
*   **Metrics:** Time taken for cloning operations (in microseconds or nanoseconds)
*   **Experimental Environment:**
    *   CPU: Apple M4 Pro
    *   RAM: 16G
    *   OS: sonoma
    *   Rust Version: rustc 1.84.1 (e71f9a9a9 2025-01-27)
*   **Benchmarking Tool:** Criterion

## 4. Experimental Setup (Run the Benchmark Yourself!)

To reproduce this benchmark on your own machine, follow these steps:

1.  **Clone the repository:**

    ```bash
    git clone <repository_url>
    cd <repository_directory>
    ```

2.  **Add Criterion dependency:**

    Navigate to the `clone_comparison` directory and add the Criterion dependency to the `Cargo.toml` file:

    ```bash
    cargo add criterion
    ```

    Alternatively, you can manually edit the `clone_comparison/Cargo.toml` file and add the following:

    ```toml
    [dev-dependencies]
    criterion = "0.5" # Or the latest version
    ```

3.  **Create a benchmark file:**

    Create a `benches` directory in the `clone_comparison` directory, if it doesn't exist. Then, create a file named `my_benchmark.rs` (or any name you prefer) inside the `benches` directory.  Paste the following code into `my_benchmark.rs`:

    ```rust
    use criterion::{criterion_group, criterion_main, Criterion};
    use std::rc::Rc;

    fn string_clone_benchmark(c: &mut Criterion) {
        let data = String::from("A").repeat(1_000_000); // 1MB String

        c.bench_function("String Clone", |b| {
            b.iter(|| data.clone());
        });
    }

    fn rc_clone_benchmark(c: &mut Criterion) {
        let data = String::from("A").repeat(1_000_000); // 1MB String
        let rc_data = Rc::new(data);

        c.bench_function("Rc<String> Clone", |b| {
            b.iter(|| Rc::clone(&rc_data));
        });
    }

    criterion_group!(benches, string_clone_benchmark, rc_clone_benchmark);
    criterion_main!(benches);
    ```

4.  **Configure the `Cargo.toml` file:**

    Edit the `clone_comparison/Cargo.toml` file to include the following `[[bench]]` section:

    ```toml
    [[bench]]
    name = "my_benchmark" # Or the name of your benchmark file
    harness = false
    ```

5.  **Run the benchmark:**

    Navigate to the root of the workspace (`rust-lab` directory if you are using a workspace) and run the following command:

    ```bash
    cargo bench --profile release
    ```

6.  **Analyze the results:**

    The benchmark results will be printed to the console. You can also generate an HTML report for more detailed analysis (see Section 5 for instructions).

## 5. Experimental Results

Benchmarking String Clone: Collecting 100 samples in estimated 5.0583 s (343k iterations) String Clone time: [14.718 µs 14.732 µs 14.749 µs] Found 17 outliers among 100 measurements (17.00%) 5 (5.00%) high mild 12 (12.00%) high severe Rc Clone time: [2.8964 ns 2.9345 ns 2.9668 ns] Found 31 outliers among 100 measurements (31.00%) 10 (10.00%) low severe 5 (5.00%) high mild 16 (16.00%) high severe

The results above show that when cloning 1MB of string data, `String` cloning took an average of 14.7 microseconds, while `Rc<String>` cloning took an average of 2.9 nanoseconds on an Apple M4 Pro with 16GB RAM running macOS Sonoma 14.4.1 and Rustc 1.78.0. This means that `Rc<String>` cloning is approximately **5000 times** faster than `String` cloning.


## 6. Conclusion and Significance

The results of this study demonstrate that `Rc<String>` cloning is significantly more efficient than `String` cloning, at least when considering a cloning-only scenario. Using `Rc` is advantageous in terms of performance and memory, especially when large amounts of data need to be shared in multiple places and when modification is not required. However, since the data accessed through `Rc<String>` is immutable, `String` cloning should be used if the data needs to be modified, or other methods such as `Rc<RefCell<String>>` should be considered. It's important to note the hardware used was an Apple M4 Pro. Results may vary on different systems.

This research helps Rust developers consider performance and data mutability when choosing a string cloning method and contributes to the optimization of Rust code by clearly presenting the efficiency of `Rc`. It also helps improve the understanding of Rust's ownership and smart pointer concepts.