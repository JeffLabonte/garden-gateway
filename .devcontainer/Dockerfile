# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.236.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends zsh neofetch bash-completion wget unzip build-essential make fontconfig

RUN  cd /usr/share/fonts/ && wget https://github.com/ryanoasis/nerd-fonts/releases/download/v2.1.0/Hack.zip && unzip Hack.zip && fc-cache -v -f