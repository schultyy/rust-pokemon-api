FROM rust:1.70.0 as builder

WORKDIR /usr/app
RUN USER=root cargo new --bin pokemon_api
WORKDIR /usr/app/pokemon_api

COPY ./Cargo.toml  ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/pokemon_api*
RUN cargo build --release
#--------
FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/app
COPY --from=builder /usr/app/pokemon_api/target/release/pokemon_api /usr/app/pokemon_api
ADD ./rocket_config.toml Rocket.toml
CMD ["/usr/app/pokemon_api"]
