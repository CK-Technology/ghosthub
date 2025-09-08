#!/bin/bash
export DATABASE_URL=postgresql://ghosthub:ghosthub@localhost:5432/ghosthub
cargo build "$@"