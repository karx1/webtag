# WebTag

WebTag allows you to edit audio metadata tags in the browser. 

Currently very work in progress, so check back soon! 

## Running

To run WebTag locally, first install `trunk` and the `wasm32-unknown-unknown` target:

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

Then,

```bash
cd /path/to/webtag
trunk serve
```

A local version of WebTag should now be accessible at `http://localhost:8080`.
