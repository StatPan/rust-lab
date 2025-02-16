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
