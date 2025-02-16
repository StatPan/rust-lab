use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

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

criterion_group!(
    benches,
    string_clone_and_modify_benchmark,
    rc_refcell_clone_and_modify_benchmark
);
criterion_main!(benches);
