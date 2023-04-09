FROM rust:1.68-alpine as build
RUN apk add musl-dev
ARG PROJECT_NAME

WORKDIR /app
RUN cargo init --bin

COPY Cargo.lock Cargo.toml ./
RUN cargo build --release
RUN rm -rf src
RUN rm ./target/release/deps/${PROJECT_NAME}*

COPY sqlx-data.json ./
COPY ./migrations ./migrations
COPY ./src ./src
RUN SQLX_OFFLINE=true cargo build --release

FROM alpine:3
ARG PROJECT_NAME
COPY --from=build /app/target/release/${PROJECT_NAME} ./app
ENTRYPOINT [ "./app" ]



