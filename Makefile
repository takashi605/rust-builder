.PHONY: init up up-postgres down build test

# 環境初期化
init:
	cp .env.mysql.example .env.mysql
	cp .env.postgres.example .env.postgres
	cp .env.api-mysql.example .env.api-mysql
	cp .env.api-postgres.example .env.api-postgres
	cd api && cargo check --workspace --all-targets --all-features

# MySQL環境でコンテナを起動
up:
	docker compose up -d

# PostgreSQL環境でコンテナを起動
up-postgres:
	docker compose \
		-f docker-compose.yml \
		-f docker-compose.api-postgres.yml \
		up -d

# コンテナを停止
down:
	docker compose down

# コンテナをビルド
build:
	docker compose build

test:
	docker compose exec api cargo test --workspace --all-targets --all-features