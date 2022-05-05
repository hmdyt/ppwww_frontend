FROM alpine
WORKDIR /workdir

RUN apk add alpine-sdk
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $PATH:/root/.cargo/bin
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

COPY ./ ./

RUN cargo build
CMD trunk serve
