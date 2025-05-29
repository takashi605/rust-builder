FROM postgres:15-alpine

# タイムゾーンを設定
ENV TZ=Asia/Tokyo

# 必要なツールをインストール
RUN apk update && \
  apk add --no-cache tzdata && \
  cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
  echo "Asia/Tokyo" > /etc/timezone

# PostgreSQLの設定ファイルをコピー
COPY ./docker/postgres/postgresql.conf /etc/postgresql/postgresql.conf

# 初期化スクリプトは /docker-entrypoint-initdb.d/ に自動的に配置されます

CMD ["postgres", "-c", "config_file=/etc/postgresql/postgresql.conf"]
