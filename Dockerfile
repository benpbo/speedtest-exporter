FROM rust:1.86-slim

EXPOSE 9798

WORKDIR /usr/src/myapp
COPY . .

# Install Speedtest CLI tool
RUN apt-get -y update
RUN apt-get -y install curl
RUN curl -s https://packagecloud.io/install/repositories/ookla/speedtest-cli/script.deb.sh | bash
RUN apt-get install speedtest

RUN cargo install --path .

CMD ["speedtest-exporter"]

