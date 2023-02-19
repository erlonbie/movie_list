FROM rust:latest as dev

# RUN apt-get update && \
#     apt-get install -y cmake wget && \
#     rm -rf /var/lib/apt/lists/*
#
# RUN apt-get update && \
#     apt-get install -y glibc-source && \
#     rm -rf /var/lib/apt/lists/*
#
# RUN ldd --version >> versao.txt
#
# RUN cat versao.txt
#
# RUN wget -c https://ftp.gnu.org/gnu/glibc/glibc-2.34.tar.gz
#
# RUN tar -zxvf glibc-2.34.tar.gz
#
# RUN cd glibc-2.34
#
# WORKDIR "/glibc-2.34"
#
# RUN ./configure --prefix=/opt/glibc
#
# RUN make && make install
#
WORKDIR /app

COPY . .

# RUN cargo build --release

RUN cargo install cargo-watch

RUN cargo install diesel_cli --no-default-features --features postgres

ENTRYPOINT diesel migration run --database-url postgresql://postgres:example@db1:5432/postgres && diesel migration run --database-url postgresql://postgres:example@db2:5432/postgres && cargo-watch -x run
# ENTRYPOINT diesel migration run --database-url postgresql://postgres:example@db1:5432/postgres

EXPOSE 8000

# CMD ["cargo", "watch", "-x", "run"]
