# matx.dev

Personal blog built with a custom static site generator in Rust.

Markdown articles with YAML frontmatter are converted to HTML using MiniJinja templates. Includes syntax highlighting via comrak/syntect.

## Build

```sh
cargo run
```

Outputs to `dist/`.

## Development

```sh
cargo run -p serve
```

Serves at `localhost:3000` with auto-rebuild on file changes.

## Deploy

Deployed to Cloudflare Workers as static assets (`wrangler.jsonc`), served on `matx.dev`.

Run the `Build and Deploy` workflow manually. It needs the `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` repository secrets. To deploy locally:

```sh
cargo run && npx wrangler deploy
```
