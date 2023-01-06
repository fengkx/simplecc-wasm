FROM mcr.microsoft.com/devcontainers/javascript-node:18-bullseye
SHELL ["/bin/bash", "-c"]
USER node
RUN sudo apt -y update && sudo apt install -y binaryen curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo "$HOME"
RUN tail -n 5 "$HOME/.bashrc"
RUN ls "$HOME/.cargo/bin"
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN export PATH="$HOME/.cargo/bin:$PATH" && rustup toolchain install stable && rustup target add wasm32-unknown-unknown && cargo install -f wasm-bindgen-cli

