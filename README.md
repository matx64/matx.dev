# 🦀 matx.dev

My personal website. It's a custom static website generator that supports blog posts.

### Build

```sh
cargo run
```

All static files are generated in the `dist` folder.


### Serving and Development

```sh
# build and serve website
cargo run -p serve

# watch for file changes
cargo-watch -c -q -w . -x "run -p serve"
```