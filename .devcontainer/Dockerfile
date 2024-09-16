FROM ghcr.io/dfinity/icp-dev-env:7

RUN rustup install stable
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install candid-extractor

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
# has to be in the same RUN as changes made to shell profile do not affect subsequent RUNs
RUN . ~/.bashrc && foundryup 
# mark all directories as safe
RUN git config --global --add safe.directory '*'
