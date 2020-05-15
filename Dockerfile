FROM rust:alpine as builder
WORKDIR /usr/src/auth
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/auth /usr/local/bin/auth
EXPOSE 8000
CMD ["auth"]
