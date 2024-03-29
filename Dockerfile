# Builder stage
# We use the latest Rust stable release as base image
FROM rust:1.77 AS builder

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

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install OpenSSL - it is dynamicaly linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS cerfificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommands openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod

# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]