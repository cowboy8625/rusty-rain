FROM rust:1.88 AS build

WORKDIR /src
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

COPY --from=build /usr/local/cargo/bin/rusty-rain /bin/

ENTRYPOINT [ "/bin/rusty-rain" ]
