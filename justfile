# Watch dev files
run:
    cargo run -- tests/fixtures/test.txt

lint:
    cargo fmt
    cargo clippy -- -D warnings

test:
    cargo test -- --show-output

release:
    cargo build --bin compress --release
    cp target/release/compress .
    chmod +x ./compress
    ./compress
