FROM node:20.6.1-bookworm-slim AS workspace

# Workspace image, don't care the user
USER root:root

SHELL ["/bin/bash", "-c"]

# Install dev dependencies & utils
RUN apt-get update && apt-get install --no-install-recommends -y \
    build-essential \
    curl \
    git \
    gnupg2 \
    jq \
    libssl-dev \
    make \
    pkg-config \
    python3-pip \
    # Tauri
    file \
    libayatana-appindicator3-dev \
    libgtk-3-dev \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    webkit2gtk-driver \
    wget \
    xauth \
    xvfb \
    # SeaORM
    sqlite3 \
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false \
    && rm -rf /var/lib/apt/lists/*

# Manual install libssl 1.1 for sea-orm-cli
RUN curl --output /tmp/libssl1.1_1.1.1f-1ubuntu2_amd64.deb http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_amd64.deb \
    && dpkg -i /tmp/libssl1.1_1.1.1f-1ubuntu2_amd64.deb \
    && rm /tmp/libssl1.1_1.1.1f-1ubuntu2_amd64.deb

# Original base directories for `rustup`, `cargo` from build stage
ENV RUSTUP_HOME="/usr/local/rustup"
ENV CARGO_HOME="/usr/local/cargo"

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none

# Update `$PATH` for Rust
ENV PATH="${CARGO_HOME}/bin:${PATH}"

# Ensure toolchain matches as expected
COPY rust-toolchain.toml ./
RUN rustup show

# Download cargo-binstall binary
ARG CARGO_BINSTALL_VERSION="v1.3.0"

RUN curl -fsSL "https://github.com/cargo-bins/cargo-binstall/releases/download/${CARGO_BINSTALL_VERSION}/cargo-binstall-$(rustc -vV | sed -n 's|host: ||p').tgz" \
    | tar --extract --gzip --directory "${CARGO_HOME}/bin"

# Change working directory
ARG WORKSPACE="/workspace"

WORKDIR "${WORKSPACE}"

# Install dev tools
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt --break-system-packages

# Download dev tools binaries
RUN cargo binstall -y --log-level debug \
    cargo-llvm-cov \
    cargo-nextest \
    cargo-udeps \
    cargo-watch \
    sea-orm-cli \
    tauri-cli \
    tauri-driver

# Enable Node package managers
RUN corepack enable && pnpm config set store-dir ~/.local/share/pnpm/store

# Download deps
COPY .npmrc package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

VOLUME ["${WORKSPACE}/target", "${WORKSPACE}/node_modules"]

# Remove existing GPG as it interrupts GPG injection by devcontainer
RUN rm -rf ~/.gnupg

RUN git config --system --add safe.directory "${WORKSPACE}"

# Gtk-WARNING **: Locale not supported by C library ...
ENV LC_ALL=C.utf8

HEALTHCHECK NONE
