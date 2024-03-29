# Cloudflare Worker for Redirects

A simple Cloudflare Worker for redirecting requests.

## Prerequisite

Since the codebase is written in Rust, you'll need to [install Rust](https://www.rust-lang.org/tools/install) to build the worker.

[Wrangler 2](https://github.com/cloudflare/wrangler2) is also required to develop and deploy this project. Do not confuse it with the older version.

## Development

Cloudflare Workers is a closed-source SAAS offering. The closest thing we can get for local development is [miniflare](https://github.com/cloudflare/miniflare). To start, install it with `npm` (do _NOT_ install with `yarn` until this [issue](https://github.com/cloudflare/miniflare/issues/454) resolves) with:

```console
$ npm install -g miniflare
```

Then, build the worker:

```console
$ wrangler publish --dry-run
```

Finally, run the compiled worker with `miniflare` with the script provided:

```console
$ ./dev.sh
```

The local worker instance should expose the service on port `8787`, which can be customized via the `--port` flag.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
