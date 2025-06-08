# このリポジトリについて
Rust でデザインパターンを実装してみた学習記録リポジトリです。
Abstract Factory パターンを使って環境ごとの DB の繋ぎ変えを実現していきます。
## 実行方法
### 前提
Docker が動作する環境
### 実行手順
1. リポジトリをクローン
2. 以下の make コマンドを実行
```sh
make init
```
3. 続けて、以下の make コマンドを実行すると mysql 環境が立ち上がります。
```sh
make up
```
postgres 環境を立ち上げる場合：
```sh
make up-postgres
```
4. 以下のコマンドでユーザーデータを取得できます。
```sh
curl localhost:8000/users
```