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
# Cargo.tomlとCargo.lockのみをコピー
COPY api/Cargo.toml api/Cargo.lock* ./

# ダミーのmain.rsを作成して依存関係をビルド
RUN mkdir -p src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  cargo build && \
  rm -rf src

# キャッシュを保持するために、targetディレクトリを明示的に保持
# ソースコードとbacon設定のコピー
COPY api/bacon.toml ./
COPY api/src ./src

# 開発時はコンテナ起動後にビルドが走る（baconが監視）
EXPOSE 8000

# baconでホットリロード
CMD ["bacon", "--headless", "run-api"]
