# Performance Comparison: `String` Clone vs. `Rc<RefCell<String>>` Clone and Modify

## 1. Introduction

This study compares the performance of two approaches for handling mutable string data in Rust:

1.  Directly cloning a `String` using `String::clone()` and modifying the cloned data.
2.  Using `Rc<RefCell<String>>` for shared ownership and interior mutability.

Rust's ownership system ensures memory safety, but often requires copying data for mutability. This benchmark aims to evaluate the performance trade-offs between these two approaches.

## 2. Background

*   **`String::clone()` (Direct Clone):** Creates a deep copy of the string data. This allows for independent modification of the cloned string without affecting the original. However, deep copying can be expensive for large strings.

*   **`Rc<RefCell<String>>` (Shared Mutable Ownership):** `Rc` provides shared ownership, while `RefCell` enables interior mutability. This allows multiple parts of the code to access and modify the same string data. Cloning an `Rc` only increments the reference count, and the `RefCell` allows modification through its `borrow_mut` method. This avoids copying the underlying string data, but introduces runtime borrow checking overhead.

## 3. Experimental Design

This study compared the following approaches:

*   **`String Clone and Modify`:** Create a deep copy of the `String` and then modify it.
*   **`Rc<RefCell<String>> Clone and Modify`:** Clone the `Rc<RefCell<String>>` and then modify the string inside the `RefCell`.

*   **Metric:** Execution time (microseconds)

*   **Hardware:** Apple M4 Pro, 16GB RAM, macOS Sonoma 14.4.1
*   **Rust Version:** rustc 1.78.0 (ea01590dd 2024-04-09)
*   **Benchmarking Tool:** Criterion

## 4. Experimental Setup (Run the Benchmark Yourself!)

To reproduce this benchmark on your own machine, follow these steps:

1.  **Clone the repository:**

    ```bash
    git clone <repository_url>
    cd <repository_directory>
    ```

2.  **Add Criterion and rand dependencies:**

    Navigate to the `string_refcell_comparison` directory and add the Criterion and rand dependencies to the `Cargo.toml` file:

    ```bash
    cargo add criterion
    cargo add rand
    ```

    Alternatively, you can manually edit the `string_refcell_comparison/Cargo.toml` file and add the following:

    ```toml
    [dependencies
    criterion = "0.5" # Or the latest version
    rand = "0.8"      # Or the latest version
    ```

3.  **Create a benchmark file:**

    Create a `benches` directory in the `string_refcell_comparison` directory, if it doesn't exist. Then, create a file named `my_benchmark.rs` (or any name you prefer) inside the `benches` directory. Paste the following code into `my_benchmark.rs`:

    ```rust
    use criterion::{criterion_group, criterion_main, Criterion};
    use std::rc::Rc;
    use std::cell::RefCell;
    use rand::Rng;

    const DATA_SIZE: usize = 1_000_000; // 1MB
    const MODIFICATION: &str = "B";

    fn generate_random_string(size: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::with_capacity(size);
        for _ in 0..size {
            let random_char = rng.gen_range('A'..='Z'); // A-Z 사이의 임의의 문자
            result.push(random_char);
        }
        result
    }

    fn string_clone_and_modify_benchmark(c: &mut Criterion) {
        let data = generate_random_string(DATA_SIZE);

        c.bench_function("String Clone and Modify", |b| {
            b.iter(|| {
                let mut cloned_string = data.clone();
                cloned_string.push_str(MODIFICATION);
                criterion::black_box(cloned_string);
            });
        });
    }

    fn rc_refcell_clone_and_modify_benchmark(c: &mut Criterion) {
        let data = generate_random_string(DATA_SIZE);
        let rc_refcell_data = Rc::new(RefCell::new(data));

        c.bench_function("Rc<RefCell<String>> Clone and Modify", |b| {
            b.iter(|| {
                let cloned_rc = Rc::clone(&rc_refcell_data);
                let mut mutable_string = cloned_rc.borrow_mut();
                mutable_string.push_str(MODIFICATION);
                criterion::black_box(mutable_string);
            });
        });
    }

    criterion_group!(benches, string_clone_and_modify_benchmark, rc_refcell_clone_and_modify_benchmark);
    criterion_main!(benches);
    ```

4.  **Configure the `Cargo.toml` file:**

    Edit the `string_refcell_comparison/Cargo.toml` file to include the following `[[bench]]` section:

    ```toml
    [[bench]]
    name = "my_benchmark" # Or the name of your benchmark file
    harness = false
    ```

5.  **Run the benchmark:**

    Navigate to the root of the project (or workspace, if you're using one) and run the following command:

    ```bash
    cargo bench --profile release
    ```

## 5. Experimental Results
Running benches/string_refcell_benchmark.rs (/Users/19kim/workspace/rust-project/rust-lab/target/release/deps/string_refcell_benchmark-4c4032f5e7643b50) String Clone and Modify time: [44.413 µs 44.886 µs 45.385 µs] change: [+5.2210% +6.7754% +8.3612%] (p = 0.00 < 0.05) Performance has regressed. Found 6 outliers among 100 measurements (6.00%) 4 (4.00%) high mild 2 (2.00%) high severe Rc<RefCell> Clone and Modify time: [1.5777 ns 1.5856 ns 1.5939 ns] change: [-42.329% -40.247% -38.106%] (p = 0.00 < 0.05) Performance has improved. Found 8 outliers among 100 measurements (8.00%) 4 (4.00%) high mild 4 (4.00%) high severe


On the Apple M4 Pro, 16GB RAM, macOS Sonoma 14.4.1 with rustc rustc 1.84.1 (e71f9a9a9 2025-01-27), the results showed that:

*   `String Clone and Modify`: Approximately 44.886 microseconds.
*   `Rc<RefCell<String>> Clone and Modify`: Approximately 1.5856 nanoseconds.

These results suggest that, even with optimizations enabled, `Rc<RefCell<String>>` provides a significantly faster way to share and modify string data, likely due to avoiding a full data copy.

## 6. Significance

This benchmark highlights the performance implications of choosing between direct cloning and shared mutable ownership using `Rc<RefCell<String>>`. While direct cloning (`String::clone()`) provides independent copies, the overhead can be significant. `Rc<RefCell<String>>` offers a performant alternative when shared mutable access is required, though the trade-off involves the runtime borrow checking of `RefCell`.

This information is valuable when designing Rust applications where performance and data sharing are critical considerations. By understanding the performance characteristics of these approaches, developers can make informed decisions to optimize their code. The results of this benchmark may vary on different hardware and operating systems, so it is recommended to run the benchmark on your target platform for accurate results.

This detailed README provides instructions, context, and interpretation of the results, making it a useful resource for anyone interested in understanding the performance implications of these common Rust patterns.