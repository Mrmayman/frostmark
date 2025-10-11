use frostmark::{MarkState, MarkWidget};
use iced::{
    widget::{self, text_editor::Content},
    Element, Length, Task,
};

#[derive(Debug, Clone)]
enum Message {
    EditedText(widget::text_editor::Action),
    ToggleMarkdown(bool),
    /// For updating the HTML renderer state.
    /// You can add an id or enum here if you have multiple states
    UpdateState,
}

struct App {
    state: MarkState,
    editor: Content,
    /// Whether to additionally support markdown
    /// alongside HTML
    markdown: bool,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditedText(a) => {
                let is_edit = a.is_edit();
                self.editor.perform(a);
                if is_edit {
                    self.reparse();
                }
            }
            Message::UpdateState => {
                self.state.update();
            }
            Message::ToggleMarkdown(t) => {
                self.markdown = t;
                self.reparse();
            }
        }
        Task::none()
    }

    fn reparse(&mut self) {
        self.state = if self.markdown {
            MarkState::with_html_and_markdown(&self.editor.text())
        } else {
            MarkState::with_html(&self.editor.text())
        };
    }

    fn view(&self) -> Element<'_, Message> {
        let toggler = widget::row![
            widget::toggler(self.markdown).on_toggle(Message::ToggleMarkdown),
            "Support Markdown"
        ]
        .spacing(10);

        let editor = widget::text_editor(&self.editor)
            .on_action(Message::EditedText)
            .height(Length::Fill);

        widget::column![
            toggler,
            widget::row![
                editor,
                widget::scrollable(
                    MarkWidget::new(&self.state).on_updating_state(|| Message::UpdateState)
                )
                .width(Length::Fill),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

fn main() {
    iced::application("Live Preview Editing", App::update, App::view)
        .run_with(|| {
            (
                App {
                    markdown: true,
                    editor: Content::with_text(DEFAULT),
                    state: MarkState::with_html_and_markdown(DEFAULT),
                },
                Task::none(),
            )
        })
        .unwrap();
}

const DEFAULT: &str = r#"
<h1>Hello from HTML</h1>
<p>Here's a paragraph. It should appear on its own line.</p>
<div>Here's a div. It should also appear on a new line.</div><br>
Here's<b> bold text, </b>and
<i> italics!</i>
<hr>

# Hello from Markdown

As `Sonic the Hedgehog` once said,
> The problem with being faster than light
> is that you live in darkness

Anyway here's a basic list:
1. Chilli
2. Pepper
3. Sauce

```
// Code block support
fn main() {
    println!("Hello, World!");
}
```
"#;
