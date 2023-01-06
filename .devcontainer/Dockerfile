FROM mcr.microsoft.com/devcontainers/javascript-node:18-bullseye

RUN sudo apt update && sudo apt install binaryen curl && npm install
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN rustup toolchain stable && rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen

