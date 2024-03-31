FROM golang:1.22.1-alpine3.18 AS builder

ARG TARGETOS
ARG TARGETARCH

RUN apk --update add gcc libc-dev coreutils

RUN mkdir -p /app
WORKDIR /app

COPY go.mod go.sum ./

RUN go mod download

ADD . /app/
RUN --mount=type=cache,id=gocache,target=/root/.cache/go-build \
    --mount=type=cache,id=gomodcache,target=/go/pkg/mod \
    GOOS=${TARGETOS} GOARCH=${TARGETARCH} go build -buildmode=pie -ldflags "-linkmode external -extldflags -static -w" -o service cmd/*.go

########################

FROM scratch

WORKDIR /var/app/
ENTRYPOINT ["/service"]

COPY --from=builder /app/service /service

LABEL maintainer="FawenYo"
