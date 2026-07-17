# Watch dev files
run:
    cargo run

lint:
    cargo fmt
    cargo clippy -- -D warnings

test:
    cargo test

release:
    cargo build --bin compress --release
    cp target/release/compress .
    chmod +x ./compress
    ./compress
