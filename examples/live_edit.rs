use frostmark::{MarkState, MarkWidget};
use iced::{
    widget::{self, text_editor::Content},
    Element, Font, Length, Task,
};

#[derive(Debug, Clone)]
enum Message {
    Nothing,
    EditedText(widget::text_editor::Action),
}

struct App {
    state: MarkState,
    editor: Content,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Nothing => {}
            Message::EditedText(a) => {
                self.editor.perform(a);
                self.state = MarkState::with_html_and_markdown(&self.editor.text())
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        widget::row![
            widget::text_editor(&self.editor)
                .on_action(Message::EditedText)
                .height(Length::Fill),
            widget::container(
                MarkWidget::new(&self.state)
                    .font_bold(Font {
                        weight: iced::font::Weight::ExtraBold,
                        ..Default::default()
                    })
                    .font_mono(Font::MONOSPACE)
                    .on_copying_text(|_| Message::Nothing)
            )
            .width(Length::Fill),
        ]
        .padding(10)
        .spacing(10)
        .into()
    }
}

fn main() {
    iced::application("Hello World", App::update, App::view)
        .run_with(|| {
            (
                App {
                    editor: Content::with_text(DEFAULT),
                    state: MarkState::with_html(DEFAULT),
                },
                Task::none(),
            )
        })
        .unwrap();
}

const DEFAULT: &str = "Type your <b>HTML or Markdown</b> here!";
