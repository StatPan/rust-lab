use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
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
    group.bench_with_input(
        BenchmarkId::new("String Clone", DATA_SIZE),
        &data,
        |b, data| b.iter(|| string_clone_and_modify(black_box(data))),
    );

    // Rc<RefCell<String>> 벤치마크
    group.bench_with_input(
        BenchmarkId::new("Rc<RefCell> Clone", DATA_SIZE),
        &rc_refcell_data,
        |b, data| b.iter(|| rc_refcell_clone_and_modify(black_box(data))),
    );
    group.finish();
}

criterion_group!(benches, benchmark_both_methods);
criterion_main!(benches);
