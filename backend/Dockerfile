FROM rust:1.83

WORKDIR /app

COPY . .

RUN cargo build --release

ENV DATABASE_URL=postgresql://postgres:postgres@postgres:5432/postgres

# Run migrations and start the application
CMD ["./target/release/pauls_demo"]

