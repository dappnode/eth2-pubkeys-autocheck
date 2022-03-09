FROM golang:1.16.15-alpine3.15 as builder
WORKDIR /usr/src/app
COPY ./src .
RUN go build -o auto-check-remote-keys main.go

