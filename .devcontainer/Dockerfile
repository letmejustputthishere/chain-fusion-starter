FROM ghcr.io/dfinity/icp-dev-env:6

# Install caddy
RUN apt -yqq install --no-install-recommends debian-keyring debian-archive-keyring apt-transport-https gpg lsof
RUN curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
RUN curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list
RUN apt -yq update
RUN apt -yqq install --no-install-recommends caddy

# Install foundry
RUN curl -L https://foundry.paradigm.xyz | bash
# has to be in the same RUN as changes made to shell profile do not affect subsequent RUNs
RUN . ~/.bashrc && foundryup 
# mark all directories as safe
RUN git config --global --add safe.directory '*'