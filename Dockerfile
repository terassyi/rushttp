FROM rust:latest

RUN apt update -y && \
	apt install -y build-essential

WORKDIR /rushttp
COPY . .
COPY src/static/ /etc/rushttp/

RUN cargo build

CMD [ "target/debug/rushttp", "9999" ]
