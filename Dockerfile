FROM rust:latest

ENV APP_ROOT=/usr/src/app
WORKDIR $APP_ROOT

RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

CMD ["bash"]
