FROM debian:bookworm-slim AS base
WORKDIR /usr/src/backend/paynetic-api

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-static /tini
RUN chmod +x /tini

FROM base AS dev
RUN apt-get update -yq \
    && apt-get install -yq \
      pkg-config \
      openssl \
      libssl-dev \
      curl \
      procps \
      inotify-tools

COPY ./backend/paynetic-api/Cargo.* ./
COPY ./backend/paynetic-api/src ./src/
COPY ./backend/*.sh ../
RUN ../limit-workspace.sh ../Cargo.toml paynetic-api lib-api
COPY ./backend/lib-api ../lib-api/
COPY ./backend/lib-types ../lib-types

ARG EXEC_ENV
ENV EXEC_ENV=$EXEC_ENV
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG DB_NAME
ENV DB_NAME=$DB_NAME
ARG DOMAIN_SUFFIX
ENV DOMAIN_SUFFIX=$DOMAIN_SUFFIX
ARG APP_API_HOST
ENV APP_API_HOST=$APP_API_HOST
ARG APP_API_PORT
ENV APP_API_PORT=$APP_API_PORT
ARG APP_API_CORS
ENV APP_API_CORS=$APP_API_CORS
ARG APP_AUTH_SECRET
ENV APP_AUTH_SECRET=$APP_AUTH_SECRET
ARG CONFIRM_SHARED_SECRET
ENV CONFIRM_SHARED_SECRET=$CONFIRM_SHARED_SECRET
ARG S3_URL
ENV S3_URL=$S3_URL
ARG S3_ACCESS_KEY_ID
ENV S3_ACCESS_KEY_ID=$S3_ACCESS_KEY_ID
ARG S3_SECRET_ACCESS_KEY
ENV S3_SECRET_ACCESS_KEY=$S3_SECRET_ACCESS_KEY
ARG SENDGRID_API_KEY
ENV SENDGRID_API_KEY=$SENDGRID_API_KEY

COPY ./backend/target/aarch64-unknown-linux-gnu/debug/libpaynetic_api.* ../target/aarch64-unknown-linux-gnu/debug/
COPY ./backend/target/aarch64-unknown-linux-gnu/debug/paynetic-api* ../target/aarch64-unknown-linux-gnu/debug/

ENTRYPOINT ["/tini", "--"]
CMD ["../run-docker.sh", "paynetic-api"]

FROM debian:bookworm-slim AS prod
WORKDIR /

ARG EXEC_ENV
ENV EXEC_ENV=$EXEC_ENV
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG DB_NAME
ENV DB_NAME=$DB_NAME
ARG DOMAIN_SUFFIX
ENV DOMAIN_SUFFIX=$DOMAIN_SUFFIX
ARG APP_API_HOST
ENV APP_API_HOST=$APP_API_HOST
ARG APP_API_PORT
ENV APP_API_PORT=$APP_API_PORT
ARG APP_API_CORS
ENV APP_API_CORS=$APP_API_CORS
ARG APP_AUTH_SECRET
ENV APP_AUTH_SECRET=$APP_AUTH_SECRET
ARG CONFIRM_SHARED_SECRET
ENV CONFIRM_SHARED_SECRET=$CONFIRM_SHARED_SECRET
ARG S3_URL
ENV S3_URL=$S3_URL
ARG S3_ACCESS_KEY_ID
ENV S3_ACCESS_KEY_ID=$S3_ACCESS_KEY_ID
ARG S3_SECRET_ACCESS_KEY
ENV S3_SECRET_ACCESS_KEY=$S3_SECRET_ACCESS_KEY
ARG SENDGRID_API_KEY
ENV SENDGRID_API_KEY=$SENDGRID_API_KEY

COPY --from=base /tini /tini
COPY --chmod=755 ./backend/target/release/paynetic-api /
ENTRYPOINT ["/tini", "--"]
CMD ["/paynetic-api"]
