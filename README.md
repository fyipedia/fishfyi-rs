# fishfyi

[![crates.io](https://img.shields.io/crates/v/fishfyi.svg)](https://crates.io/crates/fishfyi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust client for the [FishFYI](https://fishfyi.com) REST API. Fish species. Uses `reqwest` for HTTP.

> **Explore at [fishfyi.com](https://fishfyi.com)** — interactive tools and comprehensive reference.

## Install

```toml
[dependencies]
fishfyi = "0.1"
```

## Quick Start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = fishfyi::Client::new();
    let result = client.search("query")?;
    println!("{:?}", result);
    Ok(())
}
```

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **Python** | `pip install fishfyi` | [PyPI](https://pypi.org/project/fishfyi/) |
| **npm** | `npm install fishfyi` | [npm](https://www.npmjs.com/package/fishfyi) |
| **Go** | `go get github.com/fyipedia/fishfyi-go` | [pkg.go.dev](https://pkg.go.dev/github.com/fyipedia/fishfyi-go) |
| **Rust** | `cargo add fishfyi` | [crates.io](https://crates.io/crates/fishfyi) |
| **Ruby** | `gem install fishfyi` | [rubygems](https://rubygems.org/gems/fishfyi) |


## Links

- **Site**: [fishfyi.com](https://fishfyi.com)
- **API**: [fishfyi.com/api/v1/](https://fishfyi.com/api/v1/)
- **OpenAPI**: [fishfyi.com/api/v1/schema/](https://fishfyi.com/api/v1/schema/)

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## License

MIT
