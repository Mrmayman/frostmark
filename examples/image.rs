use std::collections::{HashMap, HashSet};

use frostmark::{MarkState, MarkWidget};
use iced::{
    advanced::image::Handle,
    widget::{self, image, text_editor::Content},
    Element, Length, Task,
};

use crate::image_loader::Image;

const TEXT: &str = r"Put some *image links* here. For example:

![](https://github.com/Mrmayman/quantumlauncher/raw/main/assets/icon/ql_logo.png)

> Note: For SVG support check the `large_readme` example

---

";

#[path = "shared/image_loader.rs"]
mod image_loader;

#[derive(Debug, Clone)]
enum Message {
    EditedText(widget::text_editor::Action),
    UpdateState(frostmark::UpdateMsg),
    ImageDownloaded(Result<Image, String>),
}

struct App {
    state: MarkState,
    editor: Content,

    images: HashMap<String, image::Handle>,
    images_in_progress: HashSet<String>,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditedText(a) => {
                let is_edit = a.is_edit();
                self.editor.perform(a);
                if is_edit {
                    return self.reparse();
                }
            }
            Message::UpdateState(msg) => {
                self.state.update(msg);
            }
            Message::ImageDownloaded(res) => match res {
                Ok(image) => {
                    // Note: Ignoring `image.is_svg` for now.
                    // See the `large_readme.rs` example for how
                    // to handle SVG images.
                    self.images
                        .insert(image.url, Handle::from_bytes(image.bytes));
                }
                Err(err) => {
                    eprintln!("Couldn't download image: {err}");
                }
            },
        }
        Task::none()
    }

    #[must_use]
    fn reparse(&mut self) -> Task<Message> {
        self.state = MarkState::with_html_and_markdown(&self.editor.text());
        self.download_images()
    }

    fn download_images(&mut self) -> Task<Message> {
        Task::batch(self.state.find_image_links().into_iter().map(|url| {
            if self.images_in_progress.insert(url.clone()) {
                Task::perform(image_loader::download_image(url), Message::ImageDownloaded)
            } else {
                Task::none()
            }
        }))
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        let editor = widget::text_editor(&self.editor)
            .on_action(Message::EditedText)
            .height(Length::Fill);

        widget::row![
            editor,
            widget::scrollable(
                MarkWidget::new(&self.state)
                    .on_updating_state(Message::UpdateState)
                    .on_drawing_image(|info| {
                        // Note: This example doesn't handle SVG images
                        // but they are possible to implement.
                        // - Check if url ends with ".svg"
                        // - Download to `widget::svg::Handle` and have a second HashMap
                        // - Usse the same logic elsewhere

                        if let Some(image) = self.images.get(info.url).cloned() {
                            let mut img = widget::image(image);
                            if let Some(w) = info.width {
                                img = img.width(w);
                            }
                            if let Some(h) = info.height {
                                img = img.height(h);
                            }
                            img.into()
                        } else {
                            "...".into()
                        }
                    })
            )
            .width(Length::Fill),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(
        || {
            let mut app = App {
                editor: Content::with_text(TEXT),
                state: MarkState::with_html_and_markdown(TEXT),
                images: HashMap::new(),
                images_in_progress: HashSet::new(),
            };
            let t = app.download_images();
            (app, t)
        },
        App::update,
        App::view
    ).run()
}
