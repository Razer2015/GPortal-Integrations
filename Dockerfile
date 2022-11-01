FROM curlimages/curl:latest as downloader

ARG REPO_URL
ARG TAG
ARG TARGETPLATFORM

WORKDIR /home/curl_user

RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then export ARCHITECTURE=x86_64-unknown-linux-gnu; elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then export ARCHITECTURE=aarch64-unknown-linux-gnu; else export ARCHITECTURE=aarch64-unknown-linux-gnu; fi \
    && curl -L -o gportal-integrations.tar.gz ${REPO_URL}/releases/download/${TAG}/gportal-integrations-${ARCHITECTURE}-${TAG}.tar.gz

RUN tar -xf gportal-integrations.tar.gz

FROM debian:bullseye-slim

WORKDIR /app
COPY --from=downloader /home/curl_user/gportal-integrations .
CMD ["./gportal-integrations"]