Some examples to show off functionality and act as a reference.

# Hello

![A showcase of basic HTML/Markdown features](assets/hello.png)

```sh
cargo run --example hello
```

# Live Editing

Editing the document through a text editor,
with a live preview of the result.

![](assets/live_edit.png)

```sh
cargo run --example live_edit
```

# Image

Showcases basic rendering of images.

> Note: This doesn't deal with SVG images,
> for that, see the **Large Readme** example.

```sh
cargo run --example image --features="iced/image iced/tokio"
```

# Large Readme

Renders the README of a large project ([QuantumLauncher](https://github.com/Mrmayman/quantumlauncher))
with this library, also demonstrates SVG image rendering.

```sh
cargo run --example large_readme --features="iced/image iced/tokio iced/svg"
```
