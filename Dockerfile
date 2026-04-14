FROM docker.io/rust:1-trixie AS builder

WORKDIR /workdir

COPY rust-toolchain.toml ./
RUN cargo --version

RUN --mount=type=bind,source=articles,target=articles \
    --mount=type=bind,source=scss,target=scss \
    --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=wwwroot,target=wwwroot \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=rust-toolchain.toml,target=rust-toolchain.toml \
    --mount=type=cache,target=/workdir/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo install --locked --path .


FROM debian:trixie-slim

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

COPY --from=builder usr/local/cargo/bin/stephencoakley-blog /usr/local/bin/blog
WORKDIR /var/blog
COPY articles articles
COPY wwwroot wwwroot

EXPOSE 80

CMD ["blog"]
