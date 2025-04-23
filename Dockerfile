FROM rust:alpine

RUN apk add binutils make gcc grub-bios libisoburn nasm mtools xorriso

RUN rustup default nightly
RUN rustup component add rust-src llvm-tools

RUN mkdir /kfs

WORKDIR /kfs

CMD ["make"]
