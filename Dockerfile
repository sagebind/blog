FROM docker.io/rust:1-bookworm AS builder
WORKDIR /usr/src/blog
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/blog/target/release/stephencoakley-blog /usr/local/bin/blog
WORKDIR /var/blog
COPY articles articles
COPY wwwroot wwwroot

EXPOSE 80

CMD ["blog"]
