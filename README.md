# GhostHub

**Self-Hosted MSP Management Platform - Built with Rust + WebAssembly**

![rust](https://img.shields.io/badge/Backend-Rust-orange?logo=rust)
![wasm](https://img.shields.io/badge/Frontend-WebAssembly-654ff0?logo=webassembly)
![yew](https://img.shields.io/badge/UI-Yew-green)
![postgres](https://img.shields.io/badge/Database-PostgreSQL-336791?logo=postgresql)
![docker](https://img.shields.io/badge/Deploy-Docker-2496ED?logo=docker)

---

## Overview

**GhostHub** is a modern, self-hostable MSP (Managed Service Provider) management platform built with Rust and WebAssembly. Inspired by ITFlow but designed from the ground up for performance, security, and ease of deployment.

### Key Features

- **Client Management** - Organize your clients with contacts, locations, and detailed information
- **Asset Tracking** - Track devices, servers, and infrastructure for each client  
- **Ticketing System** - Support ticket management with time tracking and billing
- **Invoicing & Billing** - Generate invoices, track payments, and manage finances
- **Self-Hosted** - Complete control over your data
- **Modern Architecture** - Built with Rust backend and WebAssembly frontend
- **Docker Ready** - Easy deployment with Docker and docker-compose

---

## Quick Start

### Using Docker (Recommended)

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ghosthub.git
cd ghosthub
```

2. Start with docker-compose:
```bash
docker-compose up -d
```

3. Access GhostHub at `http://localhost`

---

## Architecture

- **Backend**: Axum web framework with SQLx for PostgreSQL
- **Frontend**: Yew framework compiled to WebAssembly
- **Database**: PostgreSQL with proper foreign keys and indexes
- **Deployment**: Single Docker container behind nginx

### Manual Development Setup

#### Prerequisites
- Rust 1.75+
- PostgreSQL 13+
- Node.js 18+ 
- Trunk (for building frontend)

#### Backend Setup
1. Install dependencies:
```bash
cd backend
cp .env.example .env
# Edit .env with your database settings
```

2. Setup database:
```bash
# Create database and run migrations
cargo run
```

3. Start backend:
```bash
cargo run
```

#### Frontend Setup
1. Install trunk:
```bash
cargo install trunk
```

2. Build and serve frontend:
```bash
cd frontend
trunk serve
```

## Database

GhostHub uses PostgreSQL and includes the following core tables:
- `clients` - MSP customer organizations
- `contacts` - Client contact persons
- `assets` - Client devices and infrastructure  
- `tickets` - Support requests and issues
- `invoices` - Billing and financial records

## API

RESTful API with endpoints for:
- `/api/v1/clients` - Client management
- `/api/v1/tickets` - Ticket management
- `/api/v1/assets` - Asset tracking
- `/api/v1/invoices` - Billing management

## Security

- JWT-based authentication
- Password hashing with bcrypt
- SQL injection protection with SQLx
- CORS and security headers configured
- Input validation and sanitization

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

GPL-3.0 License - see LICENSE file for details.

## Acknowledgments

Inspired by the excellent ITFlow project. GhostHub aims to provide a modern, performant alternative built with Rust and WebAssembly.

