# Stage 1: Build
FROM rust:1.56-alpine as builder


# Install SQLite3 and required dependencies
# Adding libsqlite3 to ensure the SQLite library is available
RUN apk add --no-cache sqlite sqlite-dev




WORKDIR /usr/src/myapp



COPY . .



# Install Diesel CLI with SQLite feature
RUN cargo install diesel_cli --no-default-features --features sqlite

# Run migrations
RUN diesel migration run

# Build the application
RUN cargo build --release

# Stage 2: Production
FROM alpine:latest

WORKDIR /root/
COPY --from=builder /usr/src/myapp/target/release/myapp .

# Expose port 8080
EXPOSE 8080

# Define the command to run the application
CMD ["./myapp"]
