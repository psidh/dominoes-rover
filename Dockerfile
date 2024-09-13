FROM rust:latest

WORKDIR /usr/src/rust-app

COPY Cargo.toml Cargo.lock

RUN mkdir src

RUN echo   'fn main() { println!("Hello from Docker!"); }' > src/main.rs

RUN cargo build --release

RUN rm -f src/main.rs

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]