FROM lukemathwalker/cargo-chef:latest-rust-1.68.2 as chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

# 编译阶段
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
RUN cargo build --bin axum_app --release

# 运行时阶段
FROM debian:bullseye-slim AS runtime
RUN rm /bin/sh && ln -s /bin/bash /bin/sh
WORKDIR /app
# 添加https访问ca证书
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /usr/share/zoneinfo/Asia/Shanghai /usr/share/zoneinfo/Asia/Shanghai
ENV TZ Asia/Shanghai

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# 安装 curl 
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends curl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# 安装git
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends git \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder  /app/target/release/axum_app axum_app 


# copy env
COPY --from=builder /app/.env .env

# copy start.sh
COPY --from=builder /app/start.sh start.sh
RUN chmod +x start.sh

# run the binary
CMD ["./start.sh"]

# docker build -t rust_hello .
# chsh    -s /bin/bash