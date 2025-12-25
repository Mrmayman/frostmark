use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use frostmark::{MarkState, MarkWidget, UpdateMsg};
use iced::{
    widget::{self, image, svg},
    Alignment, Element, Task,
};

use crate::image_loader::Image;

#[path = "shared/image_loader.rs"]
mod image_loader;

fn main() -> iced::Result{
    iced::application(
        || {
            let page = Page::TestSuite;
            let mut app = App {
                page,
                state: MarkState::with_html_and_markdown(page.get_contents()),
                images_normal: HashMap::new(),
                images_svg: HashMap::new(),
                images_in_progress: HashSet::new(),
            };
            let t = app.download_images();
            (app, t)
        },
        App::update,
        App::view
    ).run()
}

#[derive(Debug, Clone)]
enum Message {
    UpdateState(UpdateMsg),
    OpenLink(String),
    ChangePage(Page),
    ImageDownloaded(Result<Image, String>),
}

struct App {
    page: Page,
    state: MarkState,
    images_normal: HashMap<String, image::Handle>,
    images_svg: HashMap<String, svg::Handle>,
    images_in_progress: HashSet<String>,
}

impl App {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::UpdateState(msg) => self.state.update(msg),
            Message::OpenLink(link) => {
                _ = open::that(&link);
            }
            Message::ChangePage(page) => {
                self.page = page;
                return self.reload();
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

    fn view<'a>(&'a self) -> Element<'a, Message> {
        let page_selector = widget::row![
            "Page:",
            widget::pick_list(Page::ALL, Some(self.page), |s| Message::ChangePage(s))
        ]
        .align_y(Alignment::Center)
        .spacing(10);

        widget::scrollable(
            widget::column![
                page_selector,
                widget::horizontal_rule(2),
                MarkWidget::new(&self.state)
                    .on_updating_state(|msg| Message::UpdateState(msg))
                    .on_clicking_link(Message::OpenLink)
                    .on_drawing_image(|info| self.draw_image(info)),
            ]
            .spacing(10)
            .padding(10),
        )
        .into()
    }

    fn reload(&mut self) -> Task<Message> {
        self.state = MarkState::with_html_and_markdown(self.page.get_contents());
        self.download_images()
    }

    fn draw_image(&self, info: frostmark::ImageInfo) -> Element<'static, Message> {
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Page {
    TestSuite,
    QuantumLauncher,
}

impl Page {
    const ALL: [Self; 2] = [Self::TestSuite, Self::QuantumLauncher];

    fn get_contents(&self) -> &'static str {
        match self {
            Page::TestSuite => include_str!("assets/TEST.md"),
            Page::QuantumLauncher => include_str!("assets/QL_README.md"),
        }
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Page::TestSuite => "Test Suite",
                Page::QuantumLauncher => "QuantumLauncher",
            }
        )
    }
}
