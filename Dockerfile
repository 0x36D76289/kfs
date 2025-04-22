FROM archlinux:latest

RUN pacman -Syu --noconfirm rustup binutils make gcc grub libisoburn nasm mtools

RUN rustup default nightly
RUN rustup component add rust-src llvm-tools

RUN mkdir /kfs

WORKDIR /kfs

CMD ["make"]
