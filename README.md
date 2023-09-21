<div align="center">

  <h1><code>pen-to-todoist</code></h1>

<strong>A Rust-library to convert handwritten list-items to todoist-tasks</strong>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

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

## How to use

compile with wasm-pack web (no web-pack bundle):

```
wasm-pack build --no-typescript --target web
```

Copy `pkg` folder, which contains wasm-module, into `pen-to-todojst` project.

## Running Tests

use `wasm-pack` to run tests:

```
wasm-pack test --chrome
```
