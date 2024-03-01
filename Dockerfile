FROM rust
ADD . /app
WORKDIR /app
RUN cargo build --release
RUN cp target/release/ntex_demo ./
EXPOSE 8080
CMD ["./ntex_demo"]
