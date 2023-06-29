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
EXPOSE 8000
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD [ "curl --fail http://localhost:8000/health" ]

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/app
COPY --from=builder /usr/app/pokemon_api/target/release/pokemon_api /usr/app/pokemon_api
COPY ./rocket_config.toml Rocket.toml

RUN groupadd -g 10001 appuser && \
   useradd -u 10000 -g appuser appuser \
   && chown -R appuser:appuser /usr/app

USER appuser:appuser

CMD ["/usr/app/pokemon_api"]
