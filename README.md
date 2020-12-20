<div align="center">

  <h1><code>wasm-pack-template</code></h1>

<strong>A Rust-library to convert handwritten list-items to todoist-tasks</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]
This Rust-library

- takes base64 encoded image data of handwritten list-items
- sends it to google-vision-api
- creates a Shopping-List a todoist account
- creates todoist-tasks for every item in that list

The library can be compiled to wasm.

The library expects google-credentials from which it creates a JWT.
With this JWT the library authenticates at google-cloud, which results in an access token.
With the access token every request to the vision-api is authenticated.

The library expects a todoist-api-token that is sent on every request to the todoist-api.

A client is supposed to pass:

- image data (base64 encoded)
- google-credentials json data
- todoist api-token

Based on the todoist-api token the correct account is adressed in the creation request.
Additionaliy, every user of this libray (every client) needs it's own account with google-cloud where the vision api is activated.

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
