FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
RUN apt-get update && apt-get install -y git curl binutils clang make
RUN git clone https://github.com/Homebrew/brew ~/.linuxbrew/Homebrew \
&& mkdir ~/.linuxbrew/bin \
&& ln -s ../Homebrew/bin/brew ~/.linuxbrew/bin \
&& eval $(~/.linuxbrew/bin/brew shellenv) \
&& brew --version \
&& brew install gnuplot
ENV PATH=~/.linuxbrew/bin:~/.linuxbrew/sbin:$PATH