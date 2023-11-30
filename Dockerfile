FROM python:3.10 AS rpc-py

WORKDIR /usr/src/app/concreter

COPY ./src_py ./src_py
COPY ./proto ./proto

RUN alias cls=clear && \
    pip install --upgrade pip && \
    pip install --no-cache-dir  -r ./src_py/requirements.txt && \
    export PB="./src_py" &&  \
    python -m grpc_tools.protoc -I./proto --python_out=$PB \
    --pyi_out=$PB --grpc_python_out=$PB proto/sym.proto proto/nlp.proto

CMD ["python", "./src_py/main.py"]

FROM rust AS axum_builder

WORKDIR /usr/app/src

COPY ./examples ./examples
COPY ./frontend /usr/app/frontend
COPY ./proto ./proto
COPY ./sql ./sql
COPY ./src ./src
COPY ./build.rs ./build.rs
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain.toml ./rust-toolchain.toml

ENV NODE_VERSION=20.10.0
ENV NVM_DIR=/root/.nvm
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

RUN apt-get update && apt install -y curl && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash && \
    . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION} && \
    . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION} && \
    . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION} && \
    cd /usr/app/frontend && npm ci && npm cache clean --force && cd /usr/app/src && \
    apt update && apt upgrade -y && \
    apt install -y --no-install-recommends protobuf-compiler libprotobuf-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/* && \
    rustup install nightly && \
    cargo build --release && \
    mv ./target/release/concreter /usr/app && \
    cd /usr/app && rm -rf ./src && \
    rustup self uninstall -y && \
    apt remove -y protobuf-compiler libprotobuf-dev curl

FROM ubuntu:23.10 AS axum

WORKDIR /usr/app

COPY --from=axum_builder /usr/app /usr/app

ENV NODE_VERSION=20.10.0
ENV NVM_DIR=/root/.nvm
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

RUN apt-get update && apt install -y curl && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash && \
    . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION} && \
    . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION} && \
    . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION} && \
    cd /usr/app/frontend && npm ci && npm cache clean --force && \
    apt remove -y curl

CMD ["./concreter"]