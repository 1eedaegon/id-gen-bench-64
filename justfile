test:
    cargo test

test-v:
    cargo test --no-fail-fast -- --nocapture --test-threads=1

bench:
    cargo bench
    open target/criterion/report/index.html

show:
    cargo test -- --show-output


failed:
    cargo test -- --failed --nocapture

debug:
    RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1

help:
    @just --list
