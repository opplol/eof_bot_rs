FROM rust:1.66-buster

RUN apt-get update && \
    apt-get -y install git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup component add rust-analyzer rust-src rustfmt clippy && \
    rustup toolchain install nightly --component rust-analyzer-preview && \
    cargo install cargo-edit cargo-watch && \
    rustup target add wasm32-unknown-unknown && \
    cargo install --locked trunk

