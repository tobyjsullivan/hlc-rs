FROM amd64/rust

RUN ["rustup", "target", "add", "x86_64-unknown-linux-musl"]

WORKDIR /
RUN mkdir ./target
ADD ./ ./

RUN ["cargo", "build", "--target", "x86_64-unknown-linux-musl", "--release"]
