FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

WORKDIR root

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    git \
    vim \
    protobuf-compiler \
    telnet \
    wget \
    tar \
    clang \
    pkg-config \
    make \
    libclang-dev \
    libclang-12-dev \
    jq \
    bsdmainutils \
    ncdu \
    gcc \
    git-core \
    chrony \
    liblz4-tool \
    original-awk \
    uidmap \
    dbus-user-session \
    unzip \
    libudev-dev


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV CHAIN_ID=shielded-expedition.88f17d1d14
ENV PATH="/root/.cargo/bin:${PATH}"
ENV NAMADA_TAG=v0.32.1
ENV BINARY_URL=https://github.com/anoma/namada/releases/download/$NAMADA_TAG/namada-$NAMADA_TAG-Linux-x86_64.tar.gz

RUN cd $HOME && rm -rf namada-v*Linux-x86_64* $HOME/namada-binaries && mkdir $HOME/namada-binaries && wget $BINARY_URL && \
    tar -xf namada-*-Linux-x86_64.tar.gz -C $HOME/namada-binaries --strip-components=1 && cd $HOME/namada-binaries && \
    chmod +x namada* && cp namada* /usr/local/bin/ && cd && namada --version

WORKDIR /root

RUN namada client utils join-network --chain-id $CHAIN_ID

ARG SSH_PRIVATE_KEY

RUN git clone git@github.com:0x2pilot/namada-sdk-server

WORKDIR /root/namada-sdk-server

RUN cargo build

EXPOSE 8080

CMD cargo run