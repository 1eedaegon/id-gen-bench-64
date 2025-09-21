// use criterion::{Criterion, criterion_group, criterion_main};
// use snowflake::ProcessUniqueId;
// use sonyflake::Sonyflake;
// use ulid::Ulid;
// use uuid::Uuid;

// /// Benchmarking groups for id generator(Single)
// /// 1. Snowflake
// /// 2. Sonyflake
// /// 3. UUID
// /// 4. ULID
// fn bench_snowflake_groups(c: &mut Criterion) {
//     let mut group = c.benchmark_group("snowflake");
//     group.bench_function("snowflake id", |b| b.iter(|| ProcessUniqueId::new()));
//     group.finish();
// }
// fn bench_sonyflake_groups(c: &mut Criterion) {
//     let mut group = c.benchmark_group("sonyflake");
//     let sonyflake = Sonyflake::new().unwrap();
//     group.bench_function("sonyflake id", |b| b.iter(|| sonyflake.next_id().unwrap()));
//     group.finish();
// }
// fn bench_others_groups(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Others: UUID / ULID");
//     group.bench_function("uuid v7", |b| b.iter(|| Uuid::now_v7()));
//     group.bench_function("uuid v4", |b| b.iter(|| Uuid::new_v4()));
//     group.bench_function("ulid", |b| b.iter(|| Uuid::new_v4()));
//     group.finish();
// }

// criterion_group!(
//     id_gen_bench_64,
//     bench_snowflake_groups,
//     bench_sonyflake_groups,
//     bench_others_groups
// );
// criterion_main!(id_gen_bench_64);
