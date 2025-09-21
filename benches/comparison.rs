use criterion::{Criterion, criterion_group, criterion_main};
use snowflake::ProcessUniqueId;
use sonyflake::Sonyflake;
use std::hint::black_box;
use ulid::Ulid;
use uuid::Uuid;

fn compare_all_generators(c: &mut Criterion) {
    let mut group = c.benchmark_group("id_generators_comparison");

    // snowflake
    group.bench_function("snowflake", |b| {
        b.iter(|| black_box(ProcessUniqueId::new()))
    });

    // sonyflake
    let sonyflake = Sonyflake::new().unwrap();
    group.bench_function("sonyflake", |b| {
        b.iter(|| black_box(sonyflake.next_id().unwrap()))
    });

    // UUID v7
    group.bench_function("uuid v7", |b| b.iter(|| black_box(Uuid::now_v7())));

    // UUID v4
    group.bench_function("uuid v4", |b| b.iter(|| black_box(Uuid::new_v4())));

    // ULID
    group.bench_function("ulid", |b| b.iter(|| black_box(Ulid::new())));

    group.finish();
}

criterion_group!(benches, compare_all_generators);
criterion_main!(benches);
