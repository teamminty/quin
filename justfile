test:
    cargo test --workspace

build:
    cargo build --workspace

fmt:
    rustfmt --edition 2021 --emit files ./src/**.rs