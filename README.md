# AUL - Another useless logger

Another logging library that can be used for logging to stdout

[![Latest version](https://img.shields.io/badge/crates.io-1.3.1-red)](https://crates.io/crates/aul)
[![Documentation](https://docs.rs/log/badge.svg)](https://docs.rs/aul)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_aul&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_aul)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_aul&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_aul)
[![Technical Debt](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_aul&metric=sqale_index)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_aul)
## Documentation:

* [`aul`](https://docs.rs/aul)

## Usage:

Import the library into your Cargo.toml

```toml
[dependencies]
aul = "1.3.1"
```

You can also disable the coloring feature

```toml
[dependencies]
aul = { version = "1.3.1c", features = ["no-color"] }
```

Then use the specific macro or censor specific data

```rust
use aul::warn;

fn start_server(port: u16) {
    log!(Level::TRACE,"Called Method 'start_server' with port: {} ", port);
    warn!("Important sensitive data: {}", Sens(&port))
}

```

## FAQ

Q: I wrapped my data inside the `Sens` struct, but it still shows the values? <br>
A: You need to set the env variable `SAFE_LOGGING` to `true` at any point in runtime