FROM ekidd/rust-musl-builder

ADD --chown=rust:rust . /build
WORKDIR /build

RUN cargo build --release --all-features --target=x86_64-unknown-linux-musl

FROM alpine

COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/r53ddns /usr/bin/r53ddns

WORKDIR /data
ENTRYPOINT ["/usr/bin/r53ddns"]
