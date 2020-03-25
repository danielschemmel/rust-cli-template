FROM rust
WORKDIR /project
COPY Cargo.toml Cargo.lock rust-toolchain /project/
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm src/main.rs
COPY . /project/
RUN cargo build --release

FROM rust
WORKDIR /application
COPY --from=0 /project/target/release/application /application/application

ENTRYPOINT ["/application/application"]
