FROM debian:bookworm-slim
WORKDIR /app
COPY ./target/release/backend ./backend

EXPOSE 5001
CMD ["./backend"]

