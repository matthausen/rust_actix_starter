# Build stage
FROM rust:1.55.0-buster as build
ENV PKG_CONFIG_ALLOW_CROSS=1

COPY . .

RUN cargo install --path .

RUN cargo build --release

# Run stage
FROM gcr.io/distroless/cc-debian10

COPY --from=build /target/release/auth-svc .

CMD ["/auth-svc"]