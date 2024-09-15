# Thumbor for rust

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
