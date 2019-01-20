FROM amd64/rust

WORKDIR /
RUN mkdir ./target
ADD ./ ./

RUN ["cargo", "build", "--release"]
