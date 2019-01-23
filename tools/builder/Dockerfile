FROM amd64/rust

RUN mkdir -p /build
WORKDIR /build

RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir -p ./src
RUN echo 'fn main() { println!(\"hello_world\") }' > ./src/main.rs

ADD ./Cargo.toml ./
RUN cargo build --target x86_64-unknown-linux-musl --release

ADD ./ ./
RUN cargo build --target x86_64-unknown-linux-musl --release
