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
    *   `String::clone()`: Clone the `String` data.
    *   `Rc<String>::clone()`: Wrap the `String` data with `Rc` and clone the `Rc` pointer.
*   **Metrics:** Time taken for cloning operations (in microseconds or nanoseconds) and Throughput (in MiB/s or GiB/s).
*   **Experimental Environment:**
    *   CPU: Apple M4 Pro
    *   RAM: 16GB
    *   OS: macOS Sonoma 14.4.1 
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

    Navigate to the project directory and add the Criterion dependency to the `Cargo.toml` file:

    ```bash
    cargo add criterion
    ```

    Alternatively, you can manually edit the `Cargo.toml` file and add the following:

    ```toml
    [dependencies]
    criterion = "0.5"  # Or the latest version
    rand = "0.8"

    [dev-dependencies]
    criterion = { version = "0.5", features = ["plotters"] }

    [[bench]]
    name = "my_benches"  # Or the name of your benchmark file
    harness = false
    ```

3.  **Create a benchmark file:**

    Create a `benches` directory in the project directory, if it doesn't exist. Then, create a file named `my_benches.rs` (or any name you prefer) inside the `benches` directory. Paste the following code into `my_benches.rs`:

    ```rust
    use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
    use std::rc::Rc;

    const DATA_SIZE: usize = 1_000_000; // 1MB

    fn string_clone(data: &String) -> String {
        data.clone()
    }

    fn rc_clone(data: &Rc<String>) -> Rc<String> {
        Rc::clone(data)
    }

    fn benchmark_clones(c: &mut Criterion) {
        let data = String::from("A").repeat(DATA_SIZE); // 1MB String
        let rc_data = Rc::new(data.clone());

        let mut group = c.benchmark_group("String vs Rc Clone");
        group.throughput(Throughput::Bytes(DATA_SIZE as u64));

        // String clone 벤치마크
        group.bench_with_input(BenchmarkId::new("String Clone", DATA_SIZE), &data, |b, data| {
            b.iter(|| string_clone(black_box(data)));
        });

        // Rc<String> clone 벤치마크
        group.bench_with_input(BenchmarkId::new("Rc<String> Clone", DATA_SIZE), &rc_data, |b, rc_data| {
            b.iter(|| rc_clone(black_box(rc_data)));
        });

        group.finish();
    }

    criterion_group!(benches, benchmark_clones);
    criterion_main!(benches);
    ```

4.  **Run the benchmark:**

    Navigate to the root of the project and run the following command:

    ```bash
    cargo criterion --bench string_rc_clone
    ```

5.  **Analyze the results:**

    The benchmark results will be printed to the console.  An HTML report will also be generated in the `target/criterion` directory.

## 5. Experimental Results
```bash
String vs Rc Clone/String Clone/1000000                                                                             
                        time:   [14.733 µs 14.852 µs 15.122 µs]
                        thrpt:  [61.587 GiB/s 62.707 GiB/s 63.214 GiB/s]
String vs Rc Clone/Rc<String> Clone/1000000                                                                             
                        time:   [2.4660 ns 2.4904 ns 2.5326 ns]
                        thrpt:  [367738 GiB/s 373959 GiB/s 377666 GiB/s]
```

On the Apple M4 Pro, 16GB RAM, macOS Sonoma 14.4.1 with rustc 1.84.1 (e71f9a9a9 2025-01-27)

## 6. Conclusion and Significance

The results of this study demonstrate that `Rc<String>` cloning is significantly more efficient than `String` cloning, at least when considering a cloning-only scenario. Using `Rc` is advantageous in terms of performance and memory, especially when large amounts of data need to be shared in multiple places and when modification is not required. However, since the data accessed through `Rc<String>` is immutable, `String` cloning should be used if the data needs to be modified, or other methods such as `Rc<RefCell<String>>` should be considered. It's important to note the hardware used was an Apple M4 Pro. Results may vary on different systems.

This research helps Rust developers consider performance and data mutability when choosing a string cloning method and contributes to the optimization of Rust code by clearly presenting the efficiency of `Rc`. It also helps improve the understanding of Rust's ownership and smart pointer concepts.

## 7. Report
The detailed benchmark report, including interactive charts, can be found [here](String%20vs%20Rc%20Clone/index.html).