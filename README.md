# RMS - Recipe Management System

## TOC

- [Description](#description)
- [Project Structure](#project-structure)
- [Dev Setup](#dev-setup)

## Description

RMS, a.k.a. Recipe Management System, is a full stack Rust application
for managing all your favorite home recipes!

## Project Structure

RMS is a monorepo made up of four core components.

- [backend](./backend/README.md) - API server & database interaction
- [cli](./cli/README.md) - CLI for RMS
- [common](./common/README.md) - Shared type definitions
- [frontend](./frontend/README.md)

## Dev Setup

1. Install [diesel-cli](https://github.com/diesel-rs/diesel/tree/2.2.x/diesel_cli)

```
cargo install diesel_cli --no-default-features --features postgres
```
