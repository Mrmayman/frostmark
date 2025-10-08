use frostmark::{MarkState, MarkWidget};
use iced::{widget, Element, Font, Task};

#[derive(Debug, Clone)]
enum Message {
    Nothing,
}

struct App {
    state: MarkState,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Nothing => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        widget::container(
            MarkWidget::new(&self.state)
                .font_bold(Font {
                    weight: iced::font::Weight::ExtraBold,
                    ..Default::default()
                })
                .font_mono(Font::MONOSPACE)
                .on_copying_text(|_| Message::Nothing),
        )
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

const YOUR_TEXT: &str = r"
# Hello, World!
This is a markdown renderer <b>with inline HTML support!</b>
- You can mix and match markdown and HTML together
<hr>

```rust
App {
    state: MarkState::with_html_and_markdown(YOUR_TEXT)
}
```

## Note

> <b>Fun fact</b>: This is all built on top of existing iced widgets.
>
> No new widgets were made for this.
";
