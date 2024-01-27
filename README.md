# SemVer

This is a simple Rust program that automatically bumps the correct semantic version based on the semver content(major, minor, patch) of the commit messages. <br>

## Getting Started
These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites
What things you need to install the software and how to install them:

 - Rust
 - Docker | or other such as podman

Build the project:
```bash
cargo build
```

Run the tests:
```bash
cargo test
```

#### Docker
To build and run the Docker image:
```bash
docker build -t semver .
docker run -it --rm semver
```
