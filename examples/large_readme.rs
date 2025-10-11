use std::collections::{HashMap, HashSet};

use frostmark::{MarkState, MarkWidget};
use iced::{
    widget::{self, image, svg},
    Element, Task,
};

use crate::image_loader::Image;

const TEXT: &str = include_str!("assets/QL_README.md");

#[path = "shared/image_loader.rs"]
mod image_loader;

#[derive(Debug, Clone)]
enum Message {
    UpdateState,
    OpenLink(String),
    ImageDownloaded(Result<Image, String>),
}

struct App {
    state: MarkState,
    images_normal: HashMap<String, image::Handle>,
    images_svg: HashMap<String, svg::Handle>,
    images_in_progress: HashSet<String>,
}

impl App {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::UpdateState => self.state.update(),
            Message::OpenLink(link) => {
                _ = open::that(&link);
            }
            Message::ImageDownloaded(res) => match res {
                Ok(image) => {
                    if image.is_svg {
                        self.images_svg
                            .insert(image.url, svg::Handle::from_memory(image.bytes));
                    } else {
                        self.images_normal
                            .insert(image.url, image::Handle::from_bytes(image.bytes));
                    }
                }
                Err(err) => {
                    eprintln!("Couldn't download image: {err}");
                }
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        widget::scrollable(
            widget::container(
                MarkWidget::new(&self.state)
                    .on_updating_state(|| Message::UpdateState)
                    .on_clicking_link(Message::OpenLink)
                    .on_drawing_image(|info| {
                        if let Some(image) = self.images_normal.get(info.url).cloned() {
                            let mut img = widget::image(image);
                            if let Some(w) = info.width {
                                img = img.width(w);
                            }
                            if let Some(h) = info.height {
                                img = img.height(h);
                            }
                            img.into()
                        } else if let Some(image) = self.images_svg.get(info.url).cloned() {
                            let mut img = widget::svg(image);
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
                    }),
            )
            .padding(10),
        )
        .into()
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
}

fn main() {
    iced::application("Large Readme", App::update, App::view)
        .run_with(|| {
            let mut app = App {
                state: MarkState::with_html_and_markdown(TEXT),
                images_normal: HashMap::new(),
                images_svg: HashMap::new(),
                images_in_progress: HashSet::new(),
            };
            let t = app.download_images();
            (app, t)
        })
        .unwrap();
}
