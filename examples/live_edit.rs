use frostmark::{MarkState, MarkWidget, UpdateMsg};
use iced::{
    widget::{self, text_editor::Content},
    Element, Length, Task,
};

#[derive(Debug, Clone)]
enum Message {
    EditedText(widget::text_editor::Action),
    ChangeParseMode(Mode),
    /// For updating the HTML renderer state.
    /// You can add an id or enum here if you have multiple states
    UpdateState(UpdateMsg),
}

struct App {
    state: MarkState,
    editor: Content,
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    HtmlOnly,
    MarkdownOnly,
    MarkdownAndHtml,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::HtmlOnly => write!(f, "HTML Only"),
            Mode::MarkdownOnly => write!(f, "Markdown Only"),
            Mode::MarkdownAndHtml => write!(f, "Markdown and HTML"),
        }
    }
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
            Message::UpdateState(msg) => {
                self.state.update(msg);
            }
            Message::ChangeParseMode(t) => {
                self.mode = t;
                self.reparse();
            }
        }
        Task::none()
    }

    fn reparse(&mut self) {
        let text = self.editor.text();
        self.state = match self.mode {
            Mode::HtmlOnly => MarkState::with_html(&text),
            Mode::MarkdownOnly => MarkState::with_markdown_only(&text),
            Mode::MarkdownAndHtml => MarkState::with_html_and_markdown(&text),
        };
    }

    fn view(&self) -> Element<'_, Message> {
        let toggler = widget::row![widget::pick_list(
            [Mode::MarkdownAndHtml, Mode::HtmlOnly, Mode::MarkdownOnly],
            Some(self.mode),
            Message::ChangeParseMode
        ),]
        .spacing(10);

        let editor = widget::text_editor(&self.editor)
            .on_action(Message::EditedText)
            .height(Length::Fill);

        widget::column![
            toggler,
            widget::row![
                editor,
                widget::scrollable(
                    MarkWidget::new(&self.state).on_updating_state(|msg| Message::UpdateState(msg))
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

fn main() -> iced::Result {
    iced::application(
        || {
            (
                App {
                    mode: Mode::MarkdownAndHtml,
                    editor: Content::with_text(DEFAULT),
                    state: MarkState::with_html_and_markdown(DEFAULT),
                },
                Task::none(),
            )
        }, 
        App::update,
        App::view
    ).run()
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
