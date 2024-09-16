FROM rust:1-bookworm AS builder

WORKDIR /locatarr-table-generator

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /locatarr-table-generator/target/release/locatarr-table-generator /locatarr-table-generator

ENTRYPOINT [ "/locatarr-table-generator" ]
