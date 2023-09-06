#!/bin/bash

mkdir -p ./dist
for PACKAGE_NAME in quicknet_core quicknet_client quicknet_server; do
    # PACKAGE_NAME=$(awk '/^name/{print $3}' Cargo.toml | tr -d '"')

    if [ X"$1" == X"-d" ]; then
        cargo build --package ${PACKAGE_NAME} --target $(rustup target list | awk '/musl.*installed/{print $1}')
        strip ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME}
        cp -v ./target/x86_64-unknown-linux-musl/debug/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}

    else
        cargo build --release --package ${PACKAGE_NAME} --target $(rustup target list | awk '/musl.*installed/{print $1}')
        strip ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME}
        cp -v ./target/x86_64-unknown-linux-musl/release/${PACKAGE_NAME} ./dist/${PACKAGE_NAME}

    fi
    mkdir -p ./${PACKAGE_NAME}/dist/
    cp -v ./dist/${PACKAGE_NAME} ./${PACKAGE_NAME}/dist/
done
