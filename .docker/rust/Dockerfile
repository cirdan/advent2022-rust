FROM rust:1.65

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .
RUN cargo build

CMD ["target/debug/adv2022"]
