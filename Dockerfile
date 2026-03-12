# 開発用なので Rust が入ったフルイメージを使います
FROM rust:1.93.1

# コンテナ内の作業ディレクトリ
WORKDIR /app

# ホットリロード（コード変更を検知して再起動）のために cargo-watch をインストール
RUN cargo install cargo-watch

# ソースコードは後で docker-compose でマウントするので、ここではコピーしません
# (ビルドに必要な依存関係のキャッシュ設定は一旦シンプルにします)

# 起動コマンド：cargo-watch で実行
CMD ["cargo-watch", "-x", "run"]