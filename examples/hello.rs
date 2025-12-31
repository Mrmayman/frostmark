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

fn main() -> iced::Result {
    iced::application(
        || {
            (
                App {
                    state: MarkState::with_html_and_markdown(YOUR_TEXT),
                },
                Task::none(),
            )
        },
        App::update,
        App::view
    ).run()
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
