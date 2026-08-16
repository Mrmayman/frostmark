use iced_core::Color;

/// The style of a [`crate::MarkWidget`]
/// that affects how it's rendered.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Style {
    /// Color of regular text.
    pub text_color: Option<Color>,
    /// Color of link **text**.
    ///
    /// Default: `#5A6B9E`
    pub link_color: Option<Color>,
    /// Background color for text highlights (`<mark>` element).
    ///
    /// Default: `#F7D84B`
    pub highlight_color: Option<Color>,
}
