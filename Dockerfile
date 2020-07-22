FROM rust
# build and run inside the container
WORKDIR /usr/src/hyper_server_test
COPY . .
RUN cargo install --path .
CMD [ "hyper_server_test" ]
EXPOSE 8080/tcp


