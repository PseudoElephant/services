FROM rust:latest as build
ARG binary
WORKDIR /usr/src/api-service/target
COPY . .
RUN cargo install --path src/secret-secret/functions/${binary}/ --release

FROM gcr.io/distroless/cc-debian10
ARG binary
ARG log_level
ENV RUST_LOG=${log_level}
COPY --from=build /usr/local/cargo/bin/${binary} /asset-output/bootstrap
ENTRYPOINT [ "/asset-output/bootstrap" ]
