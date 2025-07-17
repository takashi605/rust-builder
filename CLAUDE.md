# このリポジトリについて
Rust でデザインパターンを実装してみる学習記録リポジトリです。
Builder パターンでクエリビルダーを作って、環境ごとのビルダー変更等を実現していきます。

## プロジェクト要件
1. users テーブルを扱う CRUD エンドポイントを定義する
2. mysql,postgres の各リポジトリでクエリビルダーを使うようにする

## 仕様技術
使用言語：Rust
使用DB：Mysql, PostgreSQL
　DB は USAGE_DB 環境変数で切り替え
動作環境： Dockerコンテナ(Docker/Docker Compose)
その他：
　Makefile を使用して、プロジェクトの初期化やアプリケーションの起動を簡略化

## 主要な使用ライブラリ
- DB 操作
  - sqlx
- web フレームワーク
  - actix-web
- エラーハンドリング
  - anyhow

## 最終的なディレクトリ構成
※.github 等、一部のファイル/ディレクトリは省略

```
├── api/
│   ├── src/
│   │   ├── endpoints/
│   │   │   ├── mod.rs
│   │   │   └── user.rs
│   │   ├── main.rs
│   │   ├── query_builder/
│   │   │  │── mod.rs
│   │   │  │── directors/
│   │   │  │  ├── mod.rs
│   │   │  │  └── user.rs
│   │   │  └── builders/
│   │   │      ├── postgres/
│   │   │      │ └── mod.rs
│   │   │      └── mysql/
│   │   │          └── mod.rs
│   │   └── repository/
│   │       ├── mod.rs
│   │       ├── user.rs
│   │       ├── postgres/
│   │       │   ├── mod.rs
│   │       │   └── user.rs
│   │       └── mysql/
│   │           ├── mod.rs
│   │           └── user.rs
│   ├── Cargo.lock
│   └── Cargo.toml
├── docker-compose.yml
├── docker/
│   ├── Dockerfile
│   └── entrypoint.sh
└── Makefile
```

## エラーハンドリング
- エラーハンドリングは、`anyhow` クレートを使用して行います。
- 最終的なエラー処理は以下の箇所でのみ行います。
  - `api/src/main.rs` の `main` 関数内で、実行時のエラーをキャッチし、処理を中断もしくはリカバーを行います。
  - 各エンドポイントのハンドラ関数内でエラーが発生した場合は適切な HTTP ステータスコードとメッセージを返します。
- 上記以外の箇所では、エラーを `anyhow::Result` 型で返却し、呼び出し元で適切に処理されることを期待します。

## コードスタイル
- コードは Rust の公式スタイルガイドに従って書きます。

sample:
```rust
use actix_web::{web, Scope};

pub fn sample_scope() -> Scope {
  web::scope("")
    .service(handle_funcs::hello)
    .service(handle_funcs::echo)
    .service(handle_funcs::fivesix)
    .route("/hey", web::get().to(handle_funcs::manual_hello))
}

mod handle_funcs {
  use actix_web::{get, post, HttpResponse, Responder};

  #[get("/")]
  pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
  }

  #[post("/")]
  pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
  }

  #[get("/fivesix")]
  pub async fn fivesix() -> impl Responder {
    #[derive(serde::Serialize)]
    struct Numbers {
      num1: i32,
      num2: i32,
    }
    let numbers = Numbers { num1: 5, num2: 6 };
    HttpResponse::Ok().json(numbers)
  }

  pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
  }
}
```

## データベーススキーマ
- MySQL では、データベーススキーマは以下のように定義します。

```sql
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);
```

- PostgreSQL では、データベーススキーマは以下のように定義します。

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);
```

## 環境変数
- 扱う環境変数は以下の通りです。
  - `USAGE_DB`: 使用するデータベースの種類を指定します。`mysql` または `postgres` のいずれかを指定します。
  - `DATABASE_URL`: データベースへの接続 URL を指定します。MySQL と PostgreSQL で異なる形式になります。
    - MySQL: `mysql://user:password@localhost:3306/database`
    - PostgreSQL: `postgres://user:password@localhost:5432/database`
- 環境変数は `.env.mysql` 及び `.env.postgres` ファイルに定義し、プロジェクトのルートディレクトリに配置します。
- docker-compose.yml では、`env_file` オプションに `.env.mysql` を指定します。
- `.env.postgres` を使用する場合は、docker compose up コマンド実行時に `docker-compose.api-postgres.yml` を追加指定します。
  - これにより環境変数をオーバーライドします。

## コミュニケーション
- Claude Code とのやり取りは日本語で行います。
