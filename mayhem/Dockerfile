# Build Stage
FROM --platform=linux/amd64 ubuntu:22.04 as builder

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    curl build-essential pkg-config libssl-dev

# Install latest nightly Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup component add rust-src --toolchain nightly

# Install cargo-fuzz
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd acmed/fuzz && cargo +nightly fuzz build && popd" && \
    mv acmed/fuzz/target/x86_64-unknown-linux-gnu/release/parse_duration /parse_duration && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd acmed/fuzz && cargo +nightly build --release && popd" && \
    mv acmed/fuzz/target/release/parse_duration /parse_duration_no_inst && \
    echo done

# Package Stage
FROM --platform=linux/amd64 ubuntu:22.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y libssl3

COPY --from=builder /parse_duration /parse_duration_no_inst /
