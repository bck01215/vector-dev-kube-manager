
ARG RUST_VERSION=1.76.0
ARG APP_NAME=vector-dev-kube-manager



FROM node:20-slim AS frontend
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
COPY ./frontend /app
WORKDIR /app
RUN corepack enable && \
    pnpm install && \
    pnpm run build



FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.3.0 AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

COPY --from=xx / /

RUN apk add --no-cache clang lld musl-dev git file

ARG TARGETPLATFORM

# Install cross compilation build dependencies.
RUN xx-apk add --no-cache musl-dev gcc

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    xx-cargo build --locked --release --target-dir ./target && \
    cp ./target/$(xx-cargo --print-target-triple)/release/$APP_NAME /bin/server && \
    xx-verify /bin/server

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
COPY --from=frontend /assets /assets

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/bin/server"]
