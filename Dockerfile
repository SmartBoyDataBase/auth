FROM rust:1.43 as builder
WORKDIR /usr/src/auth
COPY . .
RUN cargo install --path .

FROM debian
COPY --from=builder /usr/local/cargo/bin/auth /usr/local/bin/auth
EXPOSE 8000
CMD ["auth"]
