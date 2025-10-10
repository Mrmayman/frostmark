use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use html5ever::{tendril::TendrilSink, ParseOpts};
use iced::widget::{self, text_editor::Action};
use markup5ever_rcdom::RcDom;

/// The state of the document.
///
/// - Put this in your Application struct.
/// - Use [`Self::with_html`] and [`Self::with_html_and_markdown`]
///   functions to create this.
/// - Create a new one if the document changes
///
/// ```no_run
/// # const YOUR_TEXT: &str = "";
/// # fn e() { let m =
/// MarkState::with_html_and_markdown(YOUR_TEXT)
/// # ;
/// // or if you just want HTML
/// # let m =
/// MarkState::with_html(YOUR_TEXT)
/// # ; }
/// ```
pub struct MarkState {
    pub(crate) dom: RcDom,
    pub(crate) selection_state: HashMap<String, widget::text_editor::Content>,
    pub(crate) selection_queue: Arc<Mutex<VecDeque<(String, Action)>>>,
}

impl MarkState {
    /// Processes documents containing **pure HTML**,
    /// without any Markdown support.
    ///
    /// Use this if you prioritize performance and
    /// don't need Markdown support,
    /// or if you want to avoid potential artifacts
    /// from mixing HTML and Markdown.
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // Will never panic
    pub fn with_html(input: &str) -> Self {
        let dom = html5ever::parse_document(RcDom::default(), ParseOpts::default())
            .from_utf8()
            .read_from(&mut input.as_bytes())
            // Will not panic as reading from &[u8] cannot fail
            .unwrap();

        let mut selection_state = HashMap::new();
        find_codeblocks(&dom.document, &mut selection_state, false);

        Self {
            dom,
            selection_state,
            selection_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Processes documents containing both
    /// **HTML and Markdown** (or a mix of both).
    ///
    /// Use this method when you need to support
    /// Markdown formatting. However, note that
    /// it may introduce formatting bugs when
    /// dealing with pure HTML documents.
    #[must_use]
    #[cfg(feature = "markdown")]
    pub fn with_html_and_markdown(input: &str) -> Self {
        let html = comrak::markdown_to_html(
            input,
            &comrak::Options {
                extension: comrak::ExtensionOptions {
                    strikethrough: true,
                    cjk_friendly_emphasis: true,
                    tasklist: true,
                    superscript: true,
                    subscript: true,
                    underline: true,
                    ..Default::default()
                },
                parse: comrak::ParseOptions::default(),
                render: comrak::RenderOptions {
                    // Our renderer doesn't have the
                    // vulnerabilities of a browser
                    unsafe_: true,
                    ..Default::default()
                },
            },
        );

        Self::with_html(&html)
    }

    /// Updates the internal state of the document.
    ///
    /// Call this method after receiving an update message
    /// from [`crate::MarkWidget::on_updating_state`].
    /// It currently handles the update of text selection
    /// within code blocks, but additional use cases may be
    /// supported in the future.
    pub fn update(&mut self) {
        let Some(mut actions) = self.selection_queue.lock().ok() else {
            return;
        };
        for (code, action) in actions.drain(..) {
            if let Some(n) = self.selection_state.get_mut(&code) {
                n.perform(action);
            }
        }
    }

    /// Retrieves all image URLs that need to be loaded, returned as a [`HashSet<String>`].
    ///
    /// This method gathers all image URLs in the document, which you can:
    /// 1. Download somehow (pass to an async downloader maybe?)
    /// 2. Parse using the [image](https://crates.io/crates/image) crate
    /// 3. Store as `iced::widget::image::Handle`.
    /// 4. Handle the rendering of these images via [`crate::MarkWidget::on_drawing_image`].
    pub fn find_image_links(&self) -> HashSet<String> {
        let mut storage = HashSet::new();
        find_image_links(&self.dom.document, &mut storage);
        storage
    }
}

fn find_codeblocks(
    node: &markup5ever_rcdom::Node,
    storage: &mut HashMap<String, widget::text_editor::Content>,
    scan_text: bool,
) {
    let borrow = node.children.borrow();
    match &node.data {
        markup5ever_rcdom::NodeData::Element { name, .. } if &name.local == "code" => {
            for child in &*borrow {
                find_codeblocks(child, storage, true);
            }
        }
        markup5ever_rcdom::NodeData::Text { contents } if scan_text => {
            let contents = contents.borrow().to_string();
            let v = widget::text_editor::Content::with_text(&contents);
            storage.insert(contents.clone(), v);
        }
        _ => {
            for child in &*borrow {
                find_codeblocks(child, storage, scan_text);
            }
        }
    }
}

fn find_image_links(node: &markup5ever_rcdom::Node, storage: &mut HashSet<String>) {
    let borrow = node.children.borrow();
    match &node.data {
        markup5ever_rcdom::NodeData::Element { name, attrs, .. } if &name.local == "img" => {
            let attrs = attrs.borrow();
            if let Some(attr) = attrs.iter().find(|attr| &*attr.name.local == "src") {
                let url = &*attr.value;
                if !url.is_empty() {
                    storage.insert(url.to_owned());
                }
            }
        }
        _ => {
            for child in &*borrow {
                find_image_links(child, storage);
            }
        }
    }
}
