# 🦀 matx.dev

A statically generated website. Supports markdown articles for blogging.

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
watchexec -r "cargo run -p serve"
```
