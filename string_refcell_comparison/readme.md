# Performance Comparison: `String` Clone vs. `Rc<RefCell<String>>` Clone and Modify

## 1. Introduction

This study compares the performance of two approaches for handling mutable string data in Rust:

1.  Directly cloning a `String` using `String::clone()` and modifying the cloned data.
2.  Using `Rc<RefCell<String>>` for shared ownership and interior mutability.

Rust's ownership system ensures memory safety but often requires copying data for mutability. This benchmark aims to evaluate the performance trade-offs between these two approaches.

## 2. Background

*   **`String::clone()` (Direct Clone):** Creates a deep copy of the string data. This allows for independent modification of the cloned string without affecting the original. However, deep copying can be expensive for large strings.
*   **`Rc<RefCell<String>>` (Shared Mutable Ownership):** `Rc` provides shared ownership, while `RefCell` enables interior mutability.  This allows multiple parts of the code to access and modify the same string data. Cloning an `Rc` only increments the reference count, and the `RefCell` allows modification through its `borrow_mut` method. This avoids copying the underlying string data but introduces runtime borrow checking overhead.

## 3. Experimental Design

This study compared the following approaches:

*   **`String Clone and Modify`:** Create a deep copy of the `String` and then modify it.
*   **`Rc<RefCell<String>> Clone and Modify`:** Clone the `Rc<RefCell<String>>` and then modify the string inside the `RefCell`.

*   **Metric:** Execution time (nanoseconds/microseconds)
*   **Hardware:** Apple M4 Pro, 16GB RAM, macOS Sonoma 14.4.1
*   **Rust Version:** rustc rustc 1.84.1 (e71f9a9a9 2025-01-27)
*   **Benchmarking Tool:** Criterion

## 4. Experimental Setup (Run the Benchmark Yourself!)

To reproduce this benchmark on your own machine, follow these steps:

1.  **Clone the repository:**

    ```bash
    git clone <repository_url>
    cd <repository_directory>
    ```

2.  **Add Criterion and rand dependencies:**

   Navigate to the `string_refcell_comparison` directory (프로젝트 루트) and add the Criterion and rand dependencies to the `Cargo.toml` file:

    ```bash
    cargo add criterion
    cargo add rand
    ```

    Alternatively, you can manually edit the `string_refcell_comparison/Cargo.toml` file and add the following:

    ```toml
    [dependencies]
    criterion = "0.5" # Or the latest version
    rand = "0.8"    # Or the latest version

    [dev-dependencies]
    criterion = { version = "0.5", features = ["plotters"] }

    [[bench]]
    name = "my_benchmark" # Or the name of your benchmark file
    harness = false
    ```

3.  **Create a benchmark file:**

    Create a `benches` directory in the `string_refcell_comparison` directory, if it doesn't exist. Then, create a file named `my_benchmark.rs` (or any name you prefer) inside the `benches` directory. Paste the following code into `my_benchmark.rs`:

    ```rust
    use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
    use rand::Rng;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DATA_SIZE: usize = 1_000_000; // 1MB
    const MODIFICATION: &str = "B";

    fn generate_random_string(size: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut result = String::with_capacity(size);
        for _ in 0..size {
            let random_char = rng.gen_range('A'..='Z');
            result.push(random_char);
        }
        result
    }

    fn string_clone_and_modify(data: &String) -> String {
        let mut cloned_string = data.clone();
        cloned_string.push_str(MODIFICATION);
        cloned_string
    }

    fn rc_refcell_clone_and_modify(rc_refcell_data: &Rc<RefCell<String>>) -> String {
        let cloned_rc = Rc::clone(rc_refcell_data);
        let mut mutable_string = cloned_rc.borrow_mut();
        mutable_string.push_str(MODIFICATION);
        String::from(&*mutable_string)
    }

    fn benchmark_both_methods(c: &mut Criterion) {
        let data = generate_random_string(DATA_SIZE);
        let rc_refcell_data = Rc::new(RefCell::new(data.clone()));

        let mut group = c.benchmark_group("String vs Rc<RefCell>"); // 단일 그룹
        group.throughput(Throughput::Bytes(DATA_SIZE as u64));

        // String clone 벤치마크
        group.bench_with_input(BenchmarkId::new("String Clone", DATA_SIZE), &data, |b, data| {
            b.iter(|| string_clone_and_modify(black_box(data)));
        });

        // Rc<RefCell<String>> 벤치마크
        group.bench_with_input(BenchmarkId::new("Rc<RefCell> Clone", DATA_SIZE), &rc_refcell_data, |b, data| {
            b.iter(|| rc_refcell_clone_and_modify(black_box(data)));
        });
        group.finish();

    }

    criterion_group!(benches, benchmark_both_methods);
    criterion_main!(benches);
    ```

4.  **Run the benchmark:**

    Navigate to the root of your project and run the following command:

    ```bash
    cargo criterion --bench string_refcell_comparison
    ```

## 5. Experimental Results

```bash
String vs Rc<RefCell>/String Clone/1000000                                                                             
                        time:   [44.137 µs 44.404 µs 44.693 µs]
                        thrpt:  [20.838 GiB/s 20.974 GiB/s 21.101 GiB/s]
String vs Rc<RefCell>/Rc<RefCell> Clone/1000000                                                                             
                        time:   [21.017 µs 21.318 µs 21.576 µs]
                        thrpt:  [43.165 GiB/s 43.686 GiB/s 44.314 GiB/s]
```

On the Apple M4 Pro, 16GB RAM, macOS Sonoma 14.4.1 with rustc 1.84.1 (e71f9a9a9 2025-01-27)**

## 6. Significance

This benchmark highlights the performance implications of choosing between direct cloning and shared mutable ownership using `Rc<RefCell<String>>`. While direct cloning (`String::clone()`) provides independent copies, the overhead can be significant. `Rc<RefCell<String>>` offers a performant alternative when shared mutable access is required, though the trade-off involves the runtime borrow checking of `RefCell`.

This information is valuable when designing Rust applications where performance and data sharing are critical considerations. By understanding the performance characteristics of these approaches, developers can make informed decisions to optimize their code. The results of this benchmark may vary on different hardware and operating systems, so it is recommended to run the benchmark on your target platform for accurate results.

## 7. Report

The detailed benchmark report, including interactive charts, can be found [here](string_refcell_comparison/String%20vs%20Rc_RefCell_/index.html).