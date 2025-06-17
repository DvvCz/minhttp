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

## Usage

Tested with [wrk](https://github.com/wg/wrk)

```
wrk -t12 -c400 -d10s http://localhost:3000
```

```
Running 10s test @ http://localhost:3000
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.58ms   10.14ms 211.11ms   99.33%
    Req/Sec     6.19k     1.97k   17.84k    69.54%
  742583 requests in 10.10s, 36.12MB read
  Socket errors: connect 0, read 742574, write 0, timeout 0
Requests/sec:  73526.09
Transfer/sec:      3.58MB
```