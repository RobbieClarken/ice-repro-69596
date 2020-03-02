FROM rust:1.41.0-slim-buster
RUN rustup toolchain install nightly-2020-02-16 \
    && rustup default nightly-2020-02-16
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo check
RUN sed -i -e /xyz/d src/lib.rs
ENV RUST_BACKTRACE=1
RUN cargo check
