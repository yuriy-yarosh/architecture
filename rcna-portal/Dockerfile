FROM debian:bookworm-slim AS base

ENV LANG='en_US.UTF-8' \
    LANGUAGE='en_US' \
    LC_ALL='en_US.UTF-8' \
    TZ='UTC' \
    DEBIAN_FRONTEND=noninteractive

# Keep downloaded apt packages cache for future reuse
RUN rm -f /etc/apt/apt.conf.d/docker-clean ; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

ARG BUILD_PACKAGES='zlib1g-dev libdb-dev libffi-dev libncurses-dev libsqlite3-dev libyaml-dev \
    libecpg-dev liblzma-dev libbz2-dev libreadline-dev libkrb5-dev libgss-dev libcrypto++-dev'

ARG DISTROLESS_PACKAGES=''

ARG TARGETARCH

ARG UID=1000
ARG GID=1000

RUN --mount=type=cache,id=cache-apt-${TARGETARCH},target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=lib-apt-${TARGETARCH},target=/var/lib/apt,sharing=locked \
    set -eux ; \
    apt-get update -y ; \
    apt-get upgrade -y ; \
    apt-get install -y build-essential bash git curl openssl ca-certificates locales tzdata tar gpg gnupg-agent rustup ${BUILD_PACKAGES} ${DISTROLESS_PACKAGES}; \
    echo $TZ > /etc/timezone ; \
    cp "/usr/share/zoneinfo/${TZ}" /etc/localtime || true ; \
    echo "$LANG UTF-8" >> /etc/locale.gen ; \
    locale-gen $LANG; \
    update-locale LC_CTYPE=$LANG ; \
    update-locale LANG=$LANG ; \
    update-locale LC_ALL=$LC_ALL ; \
    update-locale LANGUAGE=$LANGUAGE ; \
    dpkg-reconfigure locales ;\
    apt-get purge -y --auto-remove ; \
    find /usr -name '*.pyc' -type f -exec bash -c 'for pyc; do dpkg -S "$pyc" &> /dev/null || rm -vf "$pyc"; done' -- '{}' + ; \
    rm -rf /var/lib/apt/lists/* ; \
    mkdir -p /rcna ; \
    groupadd -g ${GID} builder ; \
    useradd --shell /bin/bash -l -m -d /rcna -u ${UID} -g ${GID} builder ; \
    chown -R ${UID}:${GID} /rcna 

WORKDIR /rcna

RUN set -eux ; \
    npm install -g corepack ; \
    corepack enable ; \
    npm install --force -g yarn 

ENV USER="developer"

SHELL ["/usr/bin/zsh", "-eo", "pipefail", "-c"]
