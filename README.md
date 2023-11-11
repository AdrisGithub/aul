# AUL - Another useless logger

Another logging library that can be used for logging to stdout

[![Latest version](https://img.shields.io/crates/v/log.svg)](https://crates.io/crates/aul)
[![Documentation](https://docs.rs/log/badge.svg)](https://docs.rs/aul)

## Documentation:

* [`aul`](https://docs.rs/aul)

## Usage:

Import it into your Cargo.toml 

```toml
[dependencies]
aul = "1.1.1"
```

You can also disable the coloring feature

```toml
[dependencies]
aul = { version = "1.1.1", features = ["no-color"] }
```

Then use the specific macro or censor specific data

```rust
use aul::level::Level;
use aul::{log,log_warn};

fn start_server(port: u16){
    log!(Level::TRACE,"Called Method 'start_server' with port: {} ", port);
    log_warn!("Important sensitive data: {}", Sens(port))
}

```

## FAQ

Q: I wrapped my data inside of the Sens struct but it still shows the values? 
A: YOu need to set the env variable `SAFE_LOGGING` to `true` at any point in runtime