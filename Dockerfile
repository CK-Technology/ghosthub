# Build stage for frontend
FROM rust:1.82 AS frontend-build

# Install trunk for building Yew apps
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY shared ./shared/
COPY frontend ./frontend/

# Build the frontend  
RUN cd frontend && trunk build --release

# Build stage for backend
FROM rust:1.82 AS backend-build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY shared ./shared/
COPY backend ./backend/

# Set environment for offline compilation  
ENV SQLX_OFFLINE=true

# Build the backend
RUN cd backend && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    nginx \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

# Copy nginx configuration
COPY docker/nginx.conf /etc/nginx/nginx.conf

# Copy frontend static files
COPY --from=frontend-build /app/frontend/dist /var/www/html

# Copy backend binary
COPY --from=backend-build /app/target/release/ghosthub-backend /usr/local/bin/ghosthub-backend

# Copy supervisor configuration
COPY docker/supervisord.conf /etc/supervisord.conf

# Copy migrations
COPY backend/migrations /app/migrations

# Create necessary directories
RUN mkdir -p /var/log/supervisor /var/run

# Set environment variables
ENV DATABASE_URL="postgresql://ghosthub:ghosthub@db:5432/ghosthub"
ENV SERVER_ADDR="127.0.0.1:8080"

EXPOSE 80
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]