FROM rust:1-alpine

RUN adduser --disabled-password --gecos "" dev
RUN apk add --update openssh bash build-base
USER dev
