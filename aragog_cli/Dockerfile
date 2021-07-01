FROM rust:slim

RUN apt update
RUN apt install -yq libssl-dev
RUN apt install -yq pkg-config

RUN cargo --version
RUN cargo install aragog_cli -qf