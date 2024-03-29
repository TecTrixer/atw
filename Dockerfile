FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin atw

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/atw /usr/local/bin
RUN apt-get update && apt-get install -y libpq5
CMD ["/usr/local/bin/atw"]
EXPOSE 3000
