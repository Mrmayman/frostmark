use std::rc::Rc;

use iced::{Length, widget};
use markup5ever_rcdom::{Node, NodeData};

use crate::{
    MarkWidget,
    renderer::{ValidTheme, get_attr},
    structs::{ChildAlignment, ChildData, ChildDataFlags, RenderedSpan},
};

impl<'a, M: Clone + 'static, T: ValidTheme + 'a> MarkWidget<'a, M, T>
where
    <T as widget::button::Catalog>::Class<'a>: From<widget::button::StyleFn<'a, T>>,
{
    pub fn draw_table(&mut self, node: &Node, data: ChildData) -> RenderedSpan<'a, M, T> {
        let mut header_cells: Vec<RenderedSpan<'a, M, T>> = Vec::new();
        let mut column_alignments: Vec<Option<ChildAlignment>> = Vec::new();
        let mut body_rows: Vec<Vec<RenderedSpan<'a, M, T>>> = Vec::new();

        let children = node.children.borrow();
        for section in children.iter() {
            let NodeData::Element { name, .. } = &section.data else {
                continue;
            };
            let section_name = name.local.to_string();

            let rows = section.children.borrow();
            for row in rows.iter() {
                let NodeData::Element { name, .. } = &row.data else {
                    continue;
                };
                if name.local.to_string() != "tr" {
                    continue;
                }

                let row_children = row.children.borrow();
                let cells: Vec<_> = row_children
                    .iter()
                    .filter(|cell| {
                        matches!(
                            &cell.data,
                            NodeData::Element { name, .. }
                                if matches!(name.local.to_string().as_str(), "th" | "td")
                        )
                    })
                    .collect();

                if section_name == "thead" || (header_cells.is_empty() && body_rows.is_empty()) {
                    // Header Cell
                    self.table_add_header_cell(
                        data,
                        &mut header_cells,
                        &mut column_alignments,
                        &cells,
                    );
                } else {
                    // Body Cell
                    body_rows.push(
                        cells
                            .iter()
                            .map(|cell| self.render_children(cell, data))
                            .collect(),
                    );
                }
            }
        }

        let body: iced::Element<'a, M, T> = widget::column(
            body_rows
                .into_iter()
                .map(|row| draw_row(row, &column_alignments).into()),
        )
        .spacing(2)
        .into();

        widget::column![
            draw_row(header_cells, &column_alignments),
            widget::rule::horizontal(1),
            body,
        ]
        .spacing(4)
        .into()
    }

    fn table_add_header_cell(
        &mut self,
        data: ChildData,
        header_cells: &mut Vec<RenderedSpan<'a, M, T>>,
        column_alignments: &mut Vec<Option<ChildAlignment>>,
        cells: &[&Rc<Node>],
    ) {
        *column_alignments = cells
            .iter()
            .map(|cell| {
                let NodeData::Element { attrs, .. } = &cell.data else {
                    return None;
                };
                let attrs = attrs.borrow();
                match get_attr(&attrs, "align") {
                    Some("right") => Some(ChildAlignment::Right),
                    Some("center" | "centre") => Some(ChildAlignment::Center),
                    _ => None,
                }
            })
            .collect();

        *header_cells = cells
            .iter()
            .map(|cell| self.render_children(cell, data.insert(ChildDataFlags::BOLD)))
            .collect();
    }
}

fn draw_row<'a, M: Clone + 'static, T: ValidTheme + 'a>(
    cells: Vec<RenderedSpan<'a, M, T>>,
    column_alignments: &[Option<ChildAlignment>],
) -> widget::Row<'a, M, T> {
    widget::row(
        cells
            .into_iter()
            .enumerate()
            .map(|(i, cell)| make_cell(cell, column_alignments.get(i).copied().flatten()).into()),
    )
    .spacing(2)
}

fn make_cell<'a, M: Clone + 'static, T: ValidTheme + 'a>(
    content: RenderedSpan<'a, M, T>,
    align: Option<ChildAlignment>,
) -> widget::Column<'a, M, T> {
    let alignment: iced::Alignment = align.map_or(iced::Alignment::Start, ChildAlignment::into);

    widget::column![content.render()]
        .align_x(alignment)
        .padding(5)
        .width(Length::Fill)
}
