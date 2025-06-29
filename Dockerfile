FROM rust:1.78-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin hn500-rs
WORKDIR /hn500-rs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/hn500*
RUN cargo build --release

# our final base
FROM rust:1.78-buster as run

# create a non-root user
RUN useradd -m appuser

# copy the build artifact from the build stage
COPY --from=build /hn500-rs/target/release/hn500-rs .
RUN chown appuser:appuser ./hn500-rs

# switch to non-root user
USER appuser

# set the startup command to run your binary
CMD ["./hn500-rs"]
