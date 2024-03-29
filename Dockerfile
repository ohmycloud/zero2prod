# We use the latest Rust stable release as base image
FROM rust:1.77

# Let's switch our working directory to `app`(equipment to `cd app`)
# The `app` folder will be created for us by Docker in case it does not exist already.
WORKDIR /app

# Install the required dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .
# To look at the saved metadata instead of trying to query a live database
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]