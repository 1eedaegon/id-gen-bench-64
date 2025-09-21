# id-gen-bench-64
For ID generator Benchmark on 64bit processor

## Comparison
Average performance measured with Criterion.rs:

| Library | Average Time | Relative Performance | Features |
|---------|--------------|---------------------|----------|
| snowflake | **~2.15 ns** | 1x (baseline) | 64-bit, Fastest, Twitter Snowflake implementation |
| sonyflake | ~39 µs | ~18,000x slower | 64-bit, Slowest, Sony's Snowflake variant |
| ulid | ~41.4 ns | ~19x slower | 128-bit |
| uuid v4 | ~440 ns | ~205x slower | 128-bit |
| uuid v7 | ~501 ns | ~233x slower | 128-bit |

## Test Environment
- Machine: MacBook Pro 14-inch (2023)
- Processor: Apple M2 Pro
- Memory: 16GB Unified Memory
- OS: macOS
- Rust Version: 1.90

## How to use
1. git clone
- `git clone git://github.com/1eedaegon/id-gen-bench-64.git`
- `cd id-gen-bench-64`

2. Run test bench with criterion
Cargo
- `cargo bench`
- `open target/criterion/report/index.html`

2.(Using just) Run test bench with criterion
Just
- `just bench`
