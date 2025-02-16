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

// 별도의 함수로 분리
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

// String clone 벤치마크 (그룹 1)
fn benchmark_string_clone(c: &mut Criterion) {
    let data = generate_random_string(DATA_SIZE); // 한 번만 생성
    let mut group = c.benchmark_group("String Clone Group"); // 그룹 이름
    group.throughput(Throughput::Bytes(DATA_SIZE as u64));
    group.bench_with_input(
        BenchmarkId::new("String Clone", DATA_SIZE),
        &data,
        |b, data| b.iter(|| string_clone_and_modify(black_box(data))),
    );
    group.finish();
}

// Rc<RefCell<String>> 벤치마크 (그룹 2)
fn benchmark_rc_refcell_clone(c: &mut Criterion) {
    let data = generate_random_string(DATA_SIZE); // 한 번만 생성
    let rc_refcell_data = Rc::new(RefCell::new(data)); // 한 번만 생성
    let mut group = c.benchmark_group("Rc<RefCell> Clone Group"); // 그룹 이름
    group.throughput(Throughput::Bytes(DATA_SIZE as u64));
    group.bench_with_input(
        BenchmarkId::new("Rc<RefCell> Clone", DATA_SIZE),
        &rc_refcell_data,
        |b, rc_refcell_data| b.iter(|| rc_refcell_clone_and_modify(black_box(rc_refcell_data))),
    );
    group.finish();
}

fn my_criterion() -> Criterion {
    Criterion::default().save_baseline("string_clone".to_string())
}

// 벤치마크 그룹 정의
criterion_group! {
    name = string_clone_group; // 그룹 이름
    config = my_criterion();  // baseline 설정
    targets = benchmark_string_clone
}
criterion_group! {
    name = rc_refcell_group;
    config = Criterion::default(); // 설정 없음
    targets = benchmark_rc_refcell_clone
}

// criterion_main!에 모든 그룹 등록
criterion_main!(string_clone_group, rc_refcell_group);
