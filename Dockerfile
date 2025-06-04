FROM debian:bookworm-slim AS build

ENV LANG='en_US.UTF-8' \
    LANGUAGE='en_US' \
    LC_ALL='en_US.UTF-8' \
    TZ='UTC' \
    DEBIAN_FRONTEND=noninteractive

RUN rm -f /etc/apt/apt.conf.d/docker-clean ; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

ARG TARGETARCH

ARG UID=1000
ARG GID=1000

RUN --mount=type=cache,id=cache-apt-${TARGETARCH},target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=lib-apt-${TARGETARCH},target=/var/lib/apt,sharing=locked \
    set -eux ; \
    apt-get update -y ; \
    apt-get upgrade -y ; \
    apt-get install -y build-essential git openssl ca-certificates locales tzdata tar gpg gnupg-agent rustup \
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
    mkdir /rcna ;\
    groupadd -g ${GID} builder ; \
    useradd --shell /bin/zsh -l -m -d /rcna -u ${UID} -g ${GID} builder ; \
    chown -R ${UID}:${GID} /rcna 

WORKDIR /rcna

RUN set -eux ; \
    rustup default nightly ; \
    rustup component add clippy-preview ; \
    rustup component add rustfmt ; \
    rustup component add miri ; \
    cargo install git-cliff --locked ; \
    cargo install cargo-nextest --locked ; \
    cargo install cargo-mutants --locked ; \
    cargo install cargo-llvm-cov --locked 

ENV USER="builder"

SHELL ["/usr/bin/zsh", "-eo", "pipefail", "-c"]
