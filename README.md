# 🧊 Frostmark

**An HTML + Markdown viewer for [iced](https://iced.rs/)**

Render rich text in your `iced` app at lightning-fast speeds using plain HTML or Markdown!

![Demo showing HTML and Markdown together](./frostmark.png)

---

## Usage

1. Create a `MarkState` and **store it in your application state**.

```rust
MarkState::with_html_and_markdown(YOUR_TEXT)
// or if you just want HTML
MarkState::with_html(YOUR_TEXT)
```

2. In your `view` function use a `MarkWidget`.

```rust
widget::container( // just an example
    MarkWidget::new(&self.mark_state)
)
.padding(10)
```

## Example

You can find runnable examples in [`examples/`](examples/)

<details>
<summary>Click to expand a fully fledged code example</summary>

```rust
use frostmark::{MarkState, MarkWidget};
use iced::{widget, Element, Task};

#[derive(Debug, Clone)]
enum Message {}

struct App {
    state: MarkState,
}

impl App {
    fn update(&mut self, _: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        widget::container(MarkWidget::new(&self.state))
            .padding(10)
            .into()
    }
}

fn main() {
    iced::application("Hello World", App::update, App::view)
        .run_with(|| {
            (
                App {
                    state: MarkState::with_html_and_markdown(YOUR_TEXT),
                },
                Task::none(),
            )
        })
        .unwrap();
}

const YOUR_TEXT: &str = "Hello from **markdown** and <b>HTML</b>!";
```

</details>
<br>

**Note:** Markdown support is optional and you can disable the `markdown`
feature to have more lightweight, HTML-only support.

## How does this work

- Markdown (if present) is converted to HTML using `comrak`.
- HTML is parsed using [`html5ever`](https://crates.io/crates/html5ever/), from the [Servo](https://servo.org/) project.
- The resulting DOM is rendered **directly to `iced` widgets** using a custom renderer.

**No custom widget types** - everything is built from standard iced components like:
`column`, `row`, `rich_text`, `button`, `horizontal_bar`, etc.

Rendering happens right inside `impl Into<Element> for MarkWidget`.

## Roadmap

- More examples (images, links, text copying)
- Better widget styling options.
- Quick “render and cache” API
- Support for sub/superscript
- Ensure stability, and publish on [crates.io](https://crates.io/)

# Contributing

This library is experimental.
Bug reports and pull requests are welcome;
contributions are appreciated!

- **License**: Dual licensed under MIT and Apache 2.0.
