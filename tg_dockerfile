FROM lukemathwalker/cargo-chef:latest-rust-1.68.2 as chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN apt-get update -y \
    && apt-get install protobuf-compiler -y
RUN cargo build --release 

FROM debian:bullseye-slim AS runtime
WORKDIR /app/hello_axum
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# COPY binaries from the builder stage  to  /app/target/release/server  and rename it to `api`
COPY --from=builder /app/target/release/hello_axum api 

EXPOSE 3000
# run the binary
CMD ["./api"]