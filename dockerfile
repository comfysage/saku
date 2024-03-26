FROM alpine as builder

RUN apk update

# █▀█ █░█ █▀ ▀█▀
# █▀▄ █▄█ ▄█ ░█░

RUN apk add rustup &&\
      rustup-init -v -y --no-modify-path --profile minimal
ENV PATH="${PATH}:/root/.cargo/bin"

# █▀▄ █▀▀ █▀█ █▀▀ █▄░█ █▀▄ █▀▀ █▄░█ █▀▀ █ █▀▀ █▀
# █▄▀ ██▄ █▀▀ ██▄ █░▀█ █▄▀ ██▄ █░▀█ █▄▄ █ ██▄ ▄█

RUN apk add --no-interactive git bash curl clang

# █ █▄░█ █▀ ▀█▀ ▄▀█ █░░ █░░ █▀▀ █▀█
# █ █░▀█ ▄█ ░█░ █▀█ █▄▄ █▄▄ ██▄ █▀▄

COPY ./install.sh .

RUN chmod u+x ./install.sh && ./install.sh

SHELL ["bash"]
CMD ["bash"]
