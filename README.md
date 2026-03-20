# fishfyi-rs

Rust client for [FishFYI](https://fishfyi.com) REST API.

## Install

```toml
[dependencies]
fishfyi = "0.1"
```

## Usage

```rust
let client = fishfyi::Client::new();
let result = client.search("query").unwrap();
println!("{:?}", result);
```

## Links

- **Site**: [https://fishfyi.com](https://fishfyi.com)
- **API**: [https://fishfyi.com/api/v1/](https://fishfyi.com/api/v1/)
- **PyPI**: [pypi.org/project/fishfyi](https://pypi.org/project/fishfyi/)
- **npm**: [npmjs.com/package/fishfyi](https://www.npmjs.com/package/fishfyi)
- **Go**: [pkg.go.dev/github.com/fyipedia/fishfyi-go](https://pkg.go.dev/github.com/fyipedia/fishfyi-go)

## License

MIT
