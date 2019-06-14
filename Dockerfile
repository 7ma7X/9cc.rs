FROM rust

RUN mkdir 9cc \
  apt install gcc make git binutils libc6-dev