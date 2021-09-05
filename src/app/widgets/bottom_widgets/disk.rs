use std::collections::HashMap;

use crossterm::event::{KeyEvent, MouseEvent};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::{
        data_farmer::DataCollection, event::WidgetEventResult,
        sort_text_table::SimpleSortableColumn, text_table::TextTableData, AppScrollWidgetState,
        CanvasTableWidthState, Component, TextTable, Widget,
    },
    canvas::Painter,
    data_conversion::convert_disk_row,
    options::layout_options::LayoutRule,
};

pub struct DiskWidgetState {
    pub scroll_state: AppScrollWidgetState,
    pub table_width_state: CanvasTableWidthState,
}

impl DiskWidgetState {
    pub fn init() -> Self {
        DiskWidgetState {
            scroll_state: AppScrollWidgetState::default(),
            table_width_state: CanvasTableWidthState::default(),
        }
    }
}

#[derive(Default)]
pub struct DiskState {
    pub widget_states: HashMap<u64, DiskWidgetState>,
}

impl DiskState {
    pub fn init(widget_states: HashMap<u64, DiskWidgetState>) -> Self {
        DiskState { widget_states }
    }

    pub fn get_mut_widget_state(&mut self, widget_id: u64) -> Option<&mut DiskWidgetState> {
        self.widget_states.get_mut(&widget_id)
    }

    pub fn get_widget_state(&self, widget_id: u64) -> Option<&DiskWidgetState> {
        self.widget_states.get(&widget_id)
    }
}

/// A table displaying disk data.
pub struct DiskTable {
    table: TextTable<SimpleSortableColumn>,
    bounds: Rect,

    display_data: TextTableData,

    width: LayoutRule,
    height: LayoutRule,
    block_border: Borders,
}

impl Default for DiskTable {
    fn default() -> Self {
        let table = TextTable::new(vec![
            SimpleSortableColumn::new_flex("Disk".into(), None, false, 0.2),
            SimpleSortableColumn::new_flex("Mount".into(), None, false, 0.2),
            SimpleSortableColumn::new_hard("Used".into(), None, false, Some(5)),
            SimpleSortableColumn::new_hard("Free".into(), None, false, Some(6)),
            SimpleSortableColumn::new_hard("Total".into(), None, false, Some(6)),
            SimpleSortableColumn::new_hard("R/s".into(), None, false, Some(7)),
            SimpleSortableColumn::new_hard("W/s".into(), None, false, Some(7)),
        ]);

        Self {
            table,
            bounds: Rect::default(),
            display_data: Default::default(),
            width: LayoutRule::default(),
            height: LayoutRule::default(),
            block_border: Borders::ALL,
        }
    }
}

impl DiskTable {
    /// Sets the width.
    pub fn width(mut self, width: LayoutRule) -> Self {
        self.width = width;
        self
    }

    /// Sets the height.
    pub fn height(mut self, height: LayoutRule) -> Self {
        self.height = height;
        self
    }

    /// Sets the block border style.
    pub fn basic_mode(mut self, basic_mode: bool) -> Self {
        if basic_mode {
            self.block_border = *crate::constants::SIDE_BORDERS;
        }

        self
    }
}

impl Component for DiskTable {
    fn handle_key_event(&mut self, event: KeyEvent) -> WidgetEventResult {
        self.table.handle_key_event(event)
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> WidgetEventResult {
        self.table.handle_mouse_event(event)
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, new_bounds: Rect) {
        self.bounds = new_bounds;
    }
}

impl Widget for DiskTable {
    fn get_pretty_name(&self) -> &'static str {
        "Disks"
    }

    fn draw<B: Backend>(
        &mut self, painter: &Painter, f: &mut Frame<'_, B>, area: Rect, selected: bool,
    ) {
        let block = Block::default()
            .border_style(if selected {
                painter.colours.highlighted_border_style
            } else {
                painter.colours.border_style
            })
            .borders(self.block_border.clone());

        self.table
            .draw_tui_table(painter, f, &self.display_data, block, area, selected);
    }

    fn update_data(&mut self, data_collection: &DataCollection) {
        self.display_data = convert_disk_row(data_collection);
    }

    fn width(&self) -> LayoutRule {
        self.width
    }

    fn height(&self) -> LayoutRule {
        self.height
    }
}