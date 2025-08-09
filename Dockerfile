FROM ubuntu:22.04

# 環境変数の設定
ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    libzmq3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && bash -c "source /usr/local/cargo/env && rustup default stable && rustup component add rust-src rust-analyzer"

# JupyterLab と evcxr (Rustカーネル) をインストール
RUN apt-get update && apt-get install -y python3 python3-pip \
    && rm -rf /var/lib/apt/lists/* \
    && python3 -m pip install --no-cache-dir --upgrade pip jupyterlab \
    && cargo install evcxr_jupyter \
    && evcxr_jupyter --install

# 作業ディレクトリの設定
WORKDIR /workspace

# Jupyter のポート
EXPOSE 8888

# デフォルトシェルの設定
CMD ["/bin/bash"] 