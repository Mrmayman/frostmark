use iced::{widget, Element, Font};
use markup5ever_rcdom::Node;

use crate::{
    structs::{ChildDataFlags, ImageInfo, MarkWidget, RenderedSpan},
    widgets::{link, link_text},
};

use super::structs::ChildData;

macro_rules! draw_children {
    ($s:expr, $node:expr, $element:expr, $child_data:expr) => {
        $s.render_children($node, $element, $child_data);
    };
}

impl<
        'a,
        M: Clone + 'static,
        T: widget::button::Catalog
            + widget::text::Catalog
            + widget::rule::Catalog
            + widget::text_editor::Catalog
            + widget::checkbox::Catalog
            + 'a,
    > MarkWidget<'a, M, T>
{
    pub(crate) fn traverse_node(
        &self,
        node: &Node,
        element: &mut RenderedSpan<'a, M, T>,
        data: ChildData,
    ) {
        match &node.data {
            markup5ever_rcdom::NodeData::Document => {
                draw_children!(self, node, element, data);
            }
            markup5ever_rcdom::NodeData::Text { contents } => {
                let text = contents.borrow();
                let weight = data.heading_weight;
                let size = if weight > 0 { 36 - (weight * 4) } else { 16 };

                *element = if data.flags.contains(ChildDataFlags::MONOSPACE) {
                    self.codeblock(
                        text.to_string(),
                        size,
                        !data.flags.contains(ChildDataFlags::KEEP_WHITESPACE),
                    )
                } else {
                    let mut t =
                        widget::span(if data.flags.contains(ChildDataFlags::KEEP_WHITESPACE) {
                            text.to_string()
                        } else {
                            clean_whitespace(&text)
                        })
                        .size(size);

                    RenderedSpan::Spans(vec![{
                        t = t.font({
                            let mut f = Font { ..self.font };
                            if data.flags.contains(ChildDataFlags::BOLD) {
                                f.weight = iced::font::Weight::Bold;
                            }
                            if data.flags.contains(ChildDataFlags::ITALIC) {
                                f.style = iced::font::Style::Italic;
                            }
                            f
                        });
                        if data.flags.contains(ChildDataFlags::STRIKETHROUGH) {
                            t = t.strikethrough(true);
                        }
                        if data.flags.contains(ChildDataFlags::UNDERLINE) {
                            t = t.underline(true);
                        }
                        t
                    }])
                };
            }
            markup5ever_rcdom::NodeData::Element { name, attrs, .. } => {
                self.render_html_inner(name, attrs, node, element, data)
            }
            _ => {}
        }
    }

    fn render_html_inner(
        &self,
        name: &html5ever::QualName,
        attrs: &std::cell::RefCell<Vec<html5ever::Attribute>>,
        node: &Node,
        element: &mut RenderedSpan<'a, M, T>,
        data: ChildData,
    ) {
        let name = name.local.to_string();
        let attrs = attrs.borrow();

        match name.as_str() {
            "center" | "kbd" | "span" | "html" | "body" | "p" | "div" => {
                draw_children!(self, node, element, data);
            }
            "pre" => {
                draw_children!(
                    self,
                    node,
                    element,
                    data.insert(ChildDataFlags::KEEP_WHITESPACE)
                );
            }
            "details" | "summary" | "h1" => {
                draw_children!(self, node, element, data.heading(1));
            }
            "h2" => {
                draw_children!(self, node, element, data.heading(2));
            }
            "h3" => {
                draw_children!(self, node, element, data.heading(3));
            }
            "h4" => {
                draw_children!(self, node, element, data.heading(4));
            }
            "blockquote" => {
                let mut t = RenderedSpan::None;
                draw_children!(self, node, &mut t, data);
                *element = widget::stack!(
                    widget::row![widget::Space::with_width(10), t.render()],
                    widget::vertical_rule(2)
                )
                .into();
            }

            "em" | "i" => {
                draw_children!(self, node, element, data.insert(ChildDataFlags::ITALIC));
            }
            "b" | "strong" => {
                draw_children!(self, node, element, data.insert(ChildDataFlags::BOLD));
            }
            "u" => {
                draw_children!(self, node, element, data.insert(ChildDataFlags::UNDERLINE));
            }
            "del" | "s" | "strike" => {
                draw_children!(
                    self,
                    node,
                    element,
                    data.insert(ChildDataFlags::STRIKETHROUGH)
                );
            }

            "a" => {
                self.draw_link(node, element, &attrs, data);
            }
            "br" => *element = widget::Column::new().into(),
            "head" | "title" | "meta" => {}
            "img" => {
                self.draw_image(element, &attrs);
            }
            "code" => {
                draw_children!(self, node, element, data.insert(ChildDataFlags::MONOSPACE));
            }
            "hr" => {
                *element = widget::horizontal_rule(4.0).into();
            }
            "ul" => {
                let mut data = data;
                data.li_ordered_number = None;
                draw_children!(self, node, element, data);
            }
            "ol" => {
                draw_children!(self, node, element, data.ordered());
            }
            "li" => {
                let bullet = if let Some(num) = data.li_ordered_number {
                    widget::text!("{num}. ")
                } else {
                    widget::text("- ")
                };
                let mut children: RenderedSpan<M, T> = widget::Column::new().into();
                draw_children!(self, node, &mut children, data);
                *element = widget::row![bullet, children.render()].into();
            }
            _ => {
                *element = widget::text!("[HTML todo: {name}]").into();
            }
        }
    }

    fn draw_image(&self, element: &mut RenderedSpan<'a, M, T>, attrs: &[html5ever::Attribute]) {
        if let Some(attr) = attrs.iter().find(|attr| &*attr.name.local == "src") {
            let url = &*attr.value;

            let width = get_attr_num(attrs, "width");
            let height = get_attr_num(attrs, "height");

            if let Some(func) = self.fn_drawing_image.as_deref() {
                *element = func(ImageInfo { url, width, height }).into();
            }
        } else {
            // Error, malformed image
            *element = RenderedSpan::None;
        }
    }

    fn draw_link(
        &self,
        node: &Node,
        element: &mut RenderedSpan<'a, M, T>,
        attrs: &std::cell::Ref<'_, Vec<html5ever::Attribute>>,
        data: ChildData,
    ) {
        *element = if let Some(attr) = attrs
            .iter()
            .find(|attr| attr.name.local.to_string().as_str() == "href")
        {
            let url = attr.value.to_string();
            let children_empty = { node.children.borrow().is_empty() };

            let mut children: RenderedSpan<M, T> = widget::Column::new().into();
            draw_children!(self, node, &mut children, data);

            let msg = self.fn_clicking_link.as_ref();
            if children_empty {
                RenderedSpan::Spans(vec![link_text(widget::span(url.clone()), &url, msg)])
            } else if let RenderedSpan::Spans(n) = children {
                RenderedSpan::Spans(n.into_iter().map(|n| link_text(n, &url, msg)).collect())
            } else {
                link(children.render(), &url, msg).into()
            }
        } else {
            let mut children: RenderedSpan<M, T> = widget::Column::new().into();
            draw_children!(self, node, &mut children, data);

            if let RenderedSpan::Spans(n) = children {
                RenderedSpan::Spans(n.into_iter().map(|n| n.underline(true)).collect())
            } else {
                link(children.render(), "", Some(&Self::e).filter(|_| false)).into()
            }
        };
    }

    fn e(_: &str) -> M {
        // This will never run, don't worry
        panic!()
    }

    fn render_children(&self, node: &Node, element: &mut RenderedSpan<'a, M, T>, data: ChildData) {
        let children = node.children.borrow();

        let mut column = Vec::new();
        let mut row = RenderedSpan::None;

        let mut i = 0;
        for item in children.iter() {
            if is_node_useless(item) {
                continue;
            }

            let mut data = data;
            if data.li_ordered_number.is_some() {
                data.li_ordered_number = Some(i + 1);
            }
            let mut element = RenderedSpan::None;
            self.traverse_node(item, &mut element, data);

            if is_block_element(item) {
                if !row.is_empty() {
                    let mut old_row = RenderedSpan::None;
                    std::mem::swap(&mut row, &mut old_row);
                    column.push(old_row);
                }

                column.push(element);
            } else {
                row = row + element;
            }

            i += 1;
        }

        if !row.is_empty() {
            column.push(row);
        }

        let len = column.len();
        let is_empty = column.is_empty() || column.iter().filter(|n| !n.is_empty()).count() == 0;

        *element = if is_empty {
            RenderedSpan::None
        } else if len == 1 {
            column.into_iter().next().unwrap()
        } else {
            widget::column(
                column
                    .into_iter()
                    .filter(|n| !n.is_empty())
                    .map(RenderedSpan::render),
            )
            .spacing(5)
            .into()
        };
    }

    fn codeblock(&self, code: String, size: u16, inline: bool) -> RenderedSpan<'a, M, T> {
        if let (false, Some(state), Some(select)) = (
            inline,
            self.state.selection_state.get(&code),
            self.fn_select.clone(),
        ) {
            let queue = self.state.selection_queue.clone();
            widget::text_editor(state)
                .size(size)
                .padding(0)
                .font(self.font_mono)
                .on_action(move |action| {
                    if !action.is_edit() {
                        queue.lock().unwrap().push_back((code.clone(), action));
                    }
                    select()
                })
                .into()
        } else {
            RenderedSpan::Spans(vec![widget::span(code).size(size).font(self.font_mono)])
        }
    }
}

fn get_attr_num(attrs: &[html5ever::Attribute], attr_name: &str) -> Option<f32> {
    get_attr(attrs, attr_name).and_then(|n| n.parse::<f32>().ok())
}

fn get_attr<'a>(attrs: &'a [html5ever::Attribute], attr_name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|attr| {
            let name = &*attr.name.local;
            name == attr_name
        })
        .map(|n| &*n.value)
}

fn is_node_useless(node: &Node) -> bool {
    if let markup5ever_rcdom::NodeData::Text { contents } = &node.data {
        let contents = contents.borrow();
        let contents = contents.to_string();
        contents.trim().is_empty()
    } else {
        false
    }
}

fn is_block_element(node: &Node) -> bool {
    let markup5ever_rcdom::NodeData::Element { name, .. } = &node.data else {
        return false;
    };
    let n: &str = &name.local;

    matches!(
        n,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "canvas"
            | "dd"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hr"
            | "li"
            | "main"
            | "nav"
            | "noscript"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "tfoot"
            | "ul"
            | "video"
            | "br"
    )
}

impl<
        'a,
        M: Clone + 'static,
        T: widget::button::Catalog
            + widget::text::Catalog
            + widget::rule::Catalog
            + widget::text_editor::Catalog
            + widget::checkbox::Catalog
            + 'a,
    > From<MarkWidget<'a, M, T>> for Element<'a, M, T>
{
    fn from(value: MarkWidget<'a, M, T>) -> Self {
        let node = &value.state.dom.document;
        let mut elem: RenderedSpan<'a, M, T> = widget::Column::new().into();
        value.traverse_node(node, &mut elem, ChildData::default());
        elem.render()
    }
}

fn clean_whitespace(input: &str) -> String {
    let mut s = input.split_whitespace().collect::<Vec<&str>>().join(" ");
    if input.ends_with(' ') {
        s.push(' ');
    }
    if input.starts_with(' ') {
        s.insert(0, ' ');
    }
    s
}
