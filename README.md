# Thumbor for rust

[![Latest Version](https://img.shields.io/crates/v/thumbor.svg)](https://crates.io/crates/thumbor)
[![Rust Documentation](https://docs.rs/thumbor/badge.svg)](https://docs.rs/thumbor)
![Crates.io](https://img.shields.io/crates/l/thumbor)
![Crates.io](https://img.shields.io/crates/d/thumbor)

## Usage

```rust
use thumbor::Server;

let server = Server::new_secured("http://localhost:8888", "my-security-key");

let builder = server.url_builder()
    .resize((300, 200))
    .smart(true)
    .build();

let url = builder.build("/path/to/my/image.jpg");
```
