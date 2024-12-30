** WIP **

Native Rust IRC client for the web with a minimal set of dependencies.


# Main Dependencies

* [pest.rs] parsing library for IRC protocol
* [yew.rs] with [web_sys] and [gloo] as web framework
* [serde.rs] for storing data
* [tailwindcss] for styling
* [heroicons] for buttons & more

[pest.rs]: https://pest.rs/
[yew.rs]: https://yew.rs/
[web_sys]: https://docs.rs/web-sys/latest/web_sys/
[gloo]: https://docs.rs/gloo/latest/gloo/
[serde.rs]: https://serde.rs/
[tailwindcss]: https://tailwindcss.com/
[heroicons]: https://heroicons.com/

There are also a bunch of development dependencies:

* [websocket] for testing IRC websocket clients against real servers
* [dotenv] for defining e.g. host address and passwords

[websocket]: https://docs.rs/websocket/latest/websocket/
[dotenv]: https://docs.rs/dotenv/latest/dotenv/

# Testing

Tests requiring a [dotenv] environment will be ignored by default, as to not
cause any issues in e.g. github actions. To run them use the following command:

```bash
cargo test -- --ignored
```

For these to work, define your own `.env` file in the source directory. As
template use:

```
WEBSOCKET_HOST="wss://..."
WEBSOCKET_PASSWORD="..."
```

Make sure to use `wss://` protocol to not transmit any clear text passwords!
