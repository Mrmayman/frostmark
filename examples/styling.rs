use frostmark::{MarkState, MarkWidget};
use iced::{Element, Task, widget};

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
        widget::container(
            MarkWidget::new(&self.state)
                .style_link_button(|t, s| {
                    // Styles link buttons to look like text links
                    widget::button::text(t, s)
                })
                .style(frostmark::Style {
                    // Example colors
                    text_color: Some(iced::Color::from_rgb8(255, 0, 0)),
                    link_color: Some(iced::Color::from_rgb8(255, 0, 255)),
                    highlight_color: Some(iced::Color::from_rgb8(0, 255, 0)),
                })
                // Difference between link buttons and link text:
                // Link buttons are links with non-text content (eg: images)
                .paragraph_spacing(20.0),
        )
        .padding(10)
        .into()
    }
}

fn main() {
    iced::application(
        || App {
            state: MarkState::with_html_and_markdown(YOUR_TEXT),
        },
        App::update,
        App::view,
    )
    .run()
    .unwrap();
}

const YOUR_TEXT: &str = r#"
This text will be red

[This text will be purple](https://example.com)

<mark>This text will be highlighted green</mark>

Lots of spacing, hmm? That's done using .paragraph_spacing(20.0)

[<div>This text is styled using .style_link_button()</div>]()
"#;
