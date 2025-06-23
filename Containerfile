# SPDX-FileCopyrightText: 2025 Chen Linxuan <me@black-desk.cn>
#
# SPDX-License-Identifier: MIT

FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev git
WORKDIR /mnt/source
COPY . .
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /mnt/source/target/release/up2date /opt/io.github.black-desk/up2date/bin/up2date

VOLUME ["/mnt"]
WORKDIR /mnt

ENTRYPOINT ["/opt/io.github.black-desk/up2date/bin/up2date"]
