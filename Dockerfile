FROM rust:alpine as build

COPY ./ /
RUN cargo build --release


FROM alpine:latest

WORKDIR /app
COPY --from=build ./target/release/iwm ./iwm

EXPOSE 8080

CMD [ "./iwm" ]