FROM rust:1.87-slim

ENV RUST_ENV=development

WORKDIR /app

# ビルド依存パッケージのインストール
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# baconのインストール
RUN cargo install bacon

# 依存関係のキャッシュ層
COPY api/Cargo.toml api/Cargo.lock* ./
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build && \
  rm -rf src

# ソースコードとbacon設定のコピー
COPY api/src ./src
COPY api/bacon.toml ./

EXPOSE 8000

# baconでホットリロード
CMD ["bacon", "--headless" ,"run-api"]
