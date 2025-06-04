FROM debian:bookworm-slim AS base

ENV LANG='en_US.UTF-8' \
    LANGUAGE='en_US' \
    LC_ALL='en_US.UTF-8' \
    TZ='UTC' \
    DEBIAN_FRONTEND=noninteractive

RUN rm -f /etc/apt/apt.conf.d/docker-clean ; \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

ARG CLI_TOOLS='procps htop mc nano'

ARG BUILD_PACKAGES='zlib1g-dev libdb-dev libffi-dev libncurses-dev libsqlite3-dev libyaml-dev \
    libecpg-dev liblzma-dev libbz2-dev libreadline-dev libkrb5-dev libgss-dev libcrypto++-dev'

ARG TARGETARCH

RUN --mount=type=cache,id=cache-apt-${TARGETARCH},target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=lib-apt-${TARGETARCH},target=/var/lib/apt,sharing=locked \
    set -eux ; \
    apt-get update -y ; \
    apt-get upgrade -y ; \
    apt-get install -y build-essential bash zsh git curl wget unzip openssl ca-certificates locales tzdata tar gpg gnupg-agent ${CLI_TOOLS} ${BUILD_PACKAGES}; \
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
    rm -rf /var/lib/apt/lists/*

ARG UID=1000
ARG GID=1000

WORKDIR /workspaces

RUN set -eux ; \
    groupadd -g ${GID} developers ; \
    useradd --shell /bin/zsh -l -m -d /workspaces -u ${UID} -g ${GID} developer ; \
    mkdir -p /workspaces/run ; \
    chmod 707 /workspaces/run ; \
    chown -R ${UID}:${GID} /workspaces 

USER developer

ARG GOLANG_VERSION="1.24.3"
ARG ASDF_VERSION="0.16.7"

RUN set -eux ; \
    mkdir -p "/workspaces/go" ; \
    wget -q https://go.dev/dl/go${GOLANG_VERSION}.linux-amd64.tar.gz  ; \
    tar xzf go${GOLANG_VERSION}.linux-amd64.tar.gz ; \
    rm go${GOLANG_VERSION}.linux-amd64.tar.gz ; \
    mv go go${GOLANG_VERSION}

# Define environment variables for the developer user
ENV GOPATH="/workspaces/go" \
    GOROOT="/workspaces/go${GOLANG_VERSION}"

ENV PATH="$GOPATH/bin:$GOROOT/bin:/workspaces/.asdf/shims:/workspaces/.local/bin:$PATH"

# Install development tools with asdf using .tool-versions file
RUN set -eux ; \
    go install -v github.com/asdf-vm/asdf/cmd/asdf@v${ASDF_VERSION} ; \
    echo "fpath=(\$HOME/.asdf/completions \$fpath)" >> /workspaces/.zshrc ; \
    echo "autoload -Uz compinit && compinit" >> /workspaces/.zshrc ; \
    echo "autoload -Uz promptinit && promptinit" >> /workspaces/.zshrc ; \
    echo "prompt walters" >> /workspaces/.zshrc ; \
    mkdir -p "$HOME/.asdf/completions" ; \
    asdf completion zsh > "$HOME/.asdf/completions/_asdf" ; \
    echo ". <(asdf completion bash)" >> /workspaces/.bashrc ; \
    asdf plugin add nodejs ; \
    asdf plugin add rust ; \
    asdf plugin add python ; \
    asdf plugin add pipx ; \
    asdf plugin add k6 ; \
    asdf plugin add trivy ; \
    asdf plugin add syft ; \
    asdf plugin add grype ; \
    asdf plugin add terraform ; \
    asdf plugin add opentofu ; \
    asdf plugin add argo ; \
    asdf plugin add argocd ; \
    asdf plugin add argo-rollouts ; \
    asdf plugin add cilium-cli ; \
    asdf plugin add cosign ; \
    asdf plugin add k9s ; \
    asdf plugin add krew ; \
    asdf plugin add kubectl ; \
    asdf plugin add kubectx ; \
    asdf plugin add helm ; \
    asdf plugin add awscli

RUN set -eux ; \
    rustup default nightly ; \
    rustup component add clippy-preview ; \
    rustup component add rustfmt ; \
    rustup component add miri ; \
    rustup component add rust-src 

ENV USER="developer"

SHELL ["/usr/bin/zsh", "-eo", "pipefail", "-c"]
