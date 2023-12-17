# Use Rust Nightly as the base image
FROM rustlang/rust:nightly

# Install SQLite3 and required dependencies
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container
COPY . .

# Install Diesel CLI with SQLite feature
RUN cargo install diesel_cli --no-default-features --features sqlite

# Run migrations
RUN diesel migration run

# Build the application
RUN cargo build --release

# Expose port 8080
EXPOSE 8080

# Define the command to run the application
CMD ["./target/release/myapp"]
