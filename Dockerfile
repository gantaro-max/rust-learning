# 開発用なので Rust が入ったフルイメージを使います
FROM rust:1.93.1

# コンテナ内の作業ディレクトリ
WORKDIR /app

#オフラインモード->DBがなくても .sqlx のデータを使ってビルドすることを許可する
ENV SQLX_OFFLINE=true

# ホットリロード（コード変更を検知して再起動）のために cargo-watch をインストール
RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features postgres

COPY . .

# 起動コマンド：マイグレーション実行とcargo-watch実行
CMD ["sh", "-c", "sqlx migrate run && cargo-watch -x run"]