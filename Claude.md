# このリポジトリについて
GraphQL 、及び Builder パターンの学習用リポジトリです。
api テスト用のクレート上で GraphQL のクエリビルダーを作っていきます。

## プロジェクト要件
1. GraphQL のリクエスト送信方法を学ぶため、学習用テスト(Learning Test)を書く。[https://countries.trevorblades.com](https://countries.trevorblades.com/) に対する api テストを書く
2. GraphQL のクエリ生成にはクエリビルダを利用する。ユースケース毎に Director を作成することで、新規クエリ生成時のテスタビリティを向上させ、ビルダー変更時の影響範囲を吸収する。

## 仕様技術
使用言語：Rust
動作環境： Dockerコンテナ(Docker/Docker Compose)

## ディレクトリ構成
```
api-request-builder(root)/
├ containers/
│   ├ api/
│   │ └ Dockerfile
│   └ api-test/
│      └ Dockerfile
├ crates/
├ compose.yaml
└ Makefile
```

## クレートの構成
```
crates/
├ schema/
├ api-server/
└ api-test/
```

### schema クレート
- .graphql ファイルに SDL を定義し、lib.rs から公開する
- 主要な依存クレートはなし

### api-server クレート
- GraphQL API サーバー
- 主要な依存クレートは以下
  - anyhow
  - async-graphql
  - actix-web
  - async-graphql-actix-web
  - tokio
  - schema(自作クレート)

### api-test クレート
- api-server クレートで立ち上げたサーバーに対して api テストを実行する
- リクエスト型・レスポンス型は schema クレート内の SDL から生成する(graphql-client を使用)
- [https://countries.trevorblades.com](https://countries.trevorblades.com/) に対する api テストを書き、基本的な GraphQL リクエストの方法を学ぶ
- 主要な依存クレートは以下
  - anyhow
  - graphql-client
  - reqwest
  - tokio
  - schema(自作クレート)

## コンテナサービス
### api-server サービス
- compose up したときに起動

### api-test サービス
- test profile を指定して compose up したときにのみ実行
- api サービスに依存している
  - api サービスが無い時期もあるので、そのときは無理に依存させる必要はない

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

## エラーハンドリング
- エラーハンドリングは、`anyhow` クレートを使用して行います。
- 最終的なエラー処理は以下の箇所でのみ行います。
  - `api/src/main.rs` の `main` 関数内で、実行時のエラーをキャッチし、処理を中断もしくはリカバーを行います。
  - 各エンドポイントのハンドラ関数内でエラーが発生した場合は適切な HTTP ステータスコードとメッセージを返します。
- 上記以外の箇所では、エラーを `anyhow::Result` 型で返却し、呼び出し元で適切に処理されることを期待します。


## 注意点
- 学習を効率的に進めるため、DB との通信は行わない。そのため、 今回は Mutation オペレーションは使わない。
