use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
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
    group.bench_with_input(
        BenchmarkId::new("String Clone", DATA_SIZE),
        &data,
        |b, data| {
            b.iter(|| string_clone(black_box(data)));
        },
    );

    // Rc<String> clone 벤치마크
    group.bench_with_input(
        BenchmarkId::new("Rc<String> Clone", DATA_SIZE),
        &rc_data,
        |b, rc_data| {
            b.iter(|| rc_clone(black_box(rc_data)));
        },
    );

    group.finish();
}

criterion_group!(benches, benchmark_clones);
criterion_main!(benches);
