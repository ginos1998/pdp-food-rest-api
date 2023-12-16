FROM rust:nightly as builder

RUN USER=root cargo new --bin pdp-food-rest-api
WORKDIR /pdp-food-rest-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/rust_microservices*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /pdp-food-rest-api/target/debug/pdp-food-rest-api /app/

ENTRYPOINT [ "/app/pdp-food-rest-api" ]
