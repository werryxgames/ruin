FROM alpine:3.18.3

RUN apk update && apk add qemu-system-x86_64 curl gcc musl-dev && curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
WORKDIR /root/ruin
ENV PATH="${PATH}:/root/.cargo/bin"
RUN rustup component add rust-src llvm-tools-preview && cargo install bootimage
COPY . ./
RUN cargo check --verbose && cargo test --verbose

