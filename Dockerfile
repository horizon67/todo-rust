FROM rust:latest

ENV APP_ROOT=/usr/src/app
WORKDIR $APP_ROOT

RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# 開発ツールをインストール
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

# Rustツールチェーンとコンポーネントをセットアップ
RUN rustup default stable && \
    rustup component add clippy

WORKDIR /app

# プロジェクトの依存関係をコピー
COPY Cargo.toml Cargo.lock ./

# ソースコードをコピー
COPY . .

RUN cargo build

CMD ["cargo", "run"]
