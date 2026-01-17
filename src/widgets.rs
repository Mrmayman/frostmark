use iced::{Element, Font, Length, advanced, widget};

use crate::structs::FStyleLinkButton;

pub fn link<'a, M: 'a, T, R: advanced::Renderer + 'a, F>(
    e: impl Into<Element<'a, M, T, R>>,
    url: &str,
    msg: Option<&F>,
    f: Option<FStyleLinkButton<T>>,
) -> widget::Button<'a, M, T, R>
where
    T: widget::button::Catalog + widget::rule::Catalog + 'a,
    F: Fn(String) -> M,
    <T as widget::button::Catalog>::Class<'a>: From<widget::button::StyleFn<'a, T>>,
{
    let mut b = widget::button(underline(e))
        .on_press_maybe(msg.map(|n| n(url.to_owned())))
        .padding(0);
    if let Some(f) = f {
        b = b.style(move |t, s| f(t, s));
    }
    b
}

pub fn link_text<'a, M: 'a, F>(
    e: widget::text::Span<'a, M, Font>,
    url: String,
    msg: Option<&F>,
) -> widget::text::Span<'a, M, Font>
where
    F: Fn(String) -> M,
{
    e.link_maybe(msg.map(|n| n(url)))
}

pub fn underline<'a, M: 'a, T: widget::rule::Catalog + 'a, R: advanced::Renderer + 'a>(
    e: impl Into<Element<'a, M, T, R>>,
) -> widget::Stack<'a, M, T, R> {
    widget::stack!(
        widget::column![e.into()],
        widget::column![
            widget::space().height(Length::Fill),
            widget::rule::horizontal(1),
            widget::space().height(1),
        ]
    )
}
