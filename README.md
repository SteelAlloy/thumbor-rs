<div align="center">
    <img src="assets/doc/logo.svg" width="256">
    <h1>Rust <a href="https://www.thumbor.org/">Thumbor</a> client</h1>
    <a href="https://github.com/SteelAlloy/thumbor-rs">
        <img
            alt="Repository"
            src="https://img.shields.io/badge/github-thumbor--rs-228b22?style=for-the-badge&labelColor=555555&logo=github"
            height="25"
    /></a>
    <a href="https://crates.io/crates/thumbor">
        <img
            alt="Crates.io Version"
            src="https://img.shields.io/crates/v/thumbor.svg?style=for-the-badge&color=e37602&logo=rust"
            height="25"
    /></a>
    <a href="https://docs.rs/thumbor/latest/thumbor">
        <img
            alt="docs.rs"
            src="https://img.shields.io/badge/docs.rs-thumbor-3b74d1?style=for-the-badge&labelColor=555555&logo=docs.rs"
            height="25"
    /></a>
    <a href="https://docs.rs/thumbor/latest/thumbor">
        <img
            alt="Crates.io MSRV"
            src="https://img.shields.io/crates/msrv/thumbor?style=for-the-badge&logo=docs.rs&color=b83fbf"
            height="25"
    /></a>
</div>

## Usage

```rust
use thumbor::Server;

let server = Server::new("http://localhost:8888", "my-security-key").unwrap();

let settings = server.settings_builder()
    .resize((300, 200))
    .smart(true)
    .build();

let url = settings.to_url("path/to/my/image.jpg");
```
