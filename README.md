# 🧊 Frostmark

**An HTML + Markdown viewer for [iced](https://iced.rs/)**

Render rich text in your `iced` app at lightning-fast speeds using plain HTML or Markdown!

![(Demo showing HTML and Markdown together)](https://github.com/Mrmayman/frostmark/raw/main/examples/assets/live_edit.png)

---

## Usage

1. Create a [`MarkState`] and **store it in your application state**.

```rust
use frostmark::MarkState;

let text = "Hello from **markdown** and <b>HTML</b>!";

let state = MarkState::with_html_and_markdown(text);
// or if you just want HTML
let state = MarkState::with_html(text);
// put this in your App struct
```

2. In your `view` function use a [`MarkWidget`].

```txt
iced::widget::container( // just an example
    MarkWidget::new(&self.mark_state)
)
.padding(10)
```

You can find runnable examples [here](examples/README.md)

<details>
<summary>Click to expand a full example</summary>

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
    iced::application(
        || App { state: MarkState::with_html_and_markdown(YOUR_TEXT) },
        App::update,
        App::view
    ).run().unwrap();
}

const YOUR_TEXT: &str = "Hello from **markdown** and <b>HTML</b>!";
```

</details>

**Note:** Markdown support is optional and you can disable the `markdown`
feature to have more lightweight, HTML-only support.

## How does this work

Markdown (if present) is converted to HTML using `comrak`.
HTML is parsed using [`html5ever`](https://crates.io/crates/html5ever/),
from the [Servo](https://servo.org/) project.
The resulting DOM is rendered **directly to `iced` widgets** using a custom renderer.

## Crate Features

All enabled by default

- `markdown` : Adds markdown support alongside HTML.
  Disable this if you want HTML-only support, or a lighter program.
- Built-in iced features (can be disabled if you wish)
  - `iced-wgpu` : wgpu rendering backend
  - `iced-tiny-skia` : tiny-skia rendering backend
  - `iced-tokio` : tokio async runtime
  - `iced-windowing` : x11 and wayland backends

## Roadmap

- Support for more elements (eg: superscript, table)

---

| Version | iced | MSRV |
|:-:|:-:|:-:|
| 0.3 | **0.14** | **1.88** |
| 0.2 | 0.13 | 1.82 |
| 0.1 | 0.13 | 1.82 |

# Contributing

This library is experimental.
Bug reports and pull requests are welcome;
contributions are appreciated!

- **License**: Dual licensed under MIT and Apache 2.0.
