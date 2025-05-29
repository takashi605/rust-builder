FROM rust:1.87-slim AS builder

WORKDIR /app

# ビルド依存パッケージのインストール
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# 依存関係のキャッシュ層
COPY api/Cargo.toml api/Cargo.lock* ./
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

# ソースコードのコピーとビルド
COPY api/src ./src
RUN cargo build --release

# 実行用イメージ
FROM debian:bookworm-slim

WORKDIR /app

# 必要なランタイム依存パッケージのインストール
RUN apt-get update && \
  apt-get install -y ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/api .

EXPOSE 8000

CMD ["./api"]
