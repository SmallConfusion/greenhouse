FROM gcr.io/distroless/static-debian12

LABEL org.opencontainers.image.source https://github.com/SmallConfusion/greenhouse

WORKDIR /app

COPY ./target/aarch64-unknown-linux-musl/release/greenhouse .

CMD ["./greenhouse", "-r"]
