FROM rust:1.82.0

# Set the working directory inside the container
WORKDIR /usr/src

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets
COPY Rocket.toml ./Rocket.toml

# This build step will cache dependencies
RUN cargo build --release
RUN rm -f target/release/onboarder*

# Copy the source code
COPY ./src ./src

# Build the application
RUN cargo install --path .

# Set the startup command to run the binary
CMD ["onboarder"]