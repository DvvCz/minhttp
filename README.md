# minhttp

This is an example of a minimal, asynchronous http server supporting TLS in Rust.

It's using
- [`smol`](https://github.com/smol-rs/smol) for the async runtime
- [`rustls`](https://github.com/rustls/rustls) for TLS support
- [`httparse`](https://github.com/seanmonstar/httparse) for HTTP parsing

## Why?

The ecosystem is completely dominated by [`tokio`](https://tokio.rs). I'm not the biggest fan due to the large amount of dependencies and disconnect from the rest of the rust ecosystem.

This was also inspired by [`may-minihttp`](https://github.com/Xudong-Huang/may_minihttp), which is similar to this, except using their stackful coroutine library, instead of an async runtime like `smol`.

## Getting Started

You can run the server without TLS as a regular development build by just running with `cargo run`.

For production, create a .env file with the format of .env.example, and create your TLS cert and key.

I recommend using `mkcert` locally.