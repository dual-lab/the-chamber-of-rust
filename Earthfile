VERSION --global-cache 0.8

IMPORT github.com/earthly/lib/rust:3.0.3 AS rust

install:
    FROM rust:1.85-bookworm
    RUN apt-get update -qq
    RUN apt-get install --no-install-recommends -qq autoconf autotools-dev libtool-bin clang cmake bsdmainutils
    RUN rustup component add clippy
    DO rust+INIT --keep_fingerprints=true

deps:
    FROM +install
    COPY --keep-ts ./Cargo.toml ./Cargo.lock .
    SAVE ARTIFACT ./Cargo.lock .
    SAVE ARTIFACT ./Cargo.toml .

rustlearning-build:
    BUILD ./rustlearning+build
