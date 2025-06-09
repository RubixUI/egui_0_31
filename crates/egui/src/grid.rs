use std::ops::{Add, Deref, Mul};
use std::usize;
use emath::GuiRounding as _;

use crate::{vec2, Align2, Color32, Context, Id, InnerResponse, NumExt, Painter, Rect, Region, Spacing, Style, Ui, UiBuilder, Vec2};

#[cfg(debug_assertions)]
use crate::Stroke;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    col_widths: Vec<f32>,
    row_heights: Vec<f32>,
}

impl State {
    pub fn load(ctx: &Context, id: Id) -> Option<Self> {
        ctx.data_mut(|d| d.get_temp(id))
    }

    pub fn store(self, ctx: &Context, id: Id) {
        // We don't persist Grids, because
        // A) there are potentially a lot of them, using up a lot of space (and therefore serialization time)
        // B) if the code changes, the grid _should_ change, and not remember old sizes
        ctx.data_mut(|d| d.insert_temp(id, self));
    }

    fn set_min_col_width(&mut self, col: usize, width: f32) {
        self.col_widths
            .resize(self.col_widths.len().max(col + 1), 0.0);
        self.col_widths[col] = self.col_widths[col].max(width);
    }

    fn set_min_row_height(&mut self, row: usize, height: f32) {
        self.row_heights
            .resize(self.row_heights.len().max(row + 1), 0.0);
        self.row_heights[row] = self.row_heights[row].max(height);
    }

    fn col_width(&self, col: usize) -> Option<f32> {
        self.col_widths.get(col).copied()
    }

    fn row_height(&self, row: usize) -> Option<f32> {
        self.row_heights.get(row).copied()
    }

    fn full_width(&self, x_spacing: f32) -> f32 {
        self.col_widths.iter().sum::<f32>()
            + (self.col_widths.len().at_least(1) - 1) as f32 * x_spacing
    }
}

pub enum GridLayoutSize {
    Collapsing(Vec2,f32),//(min_size,max_width)
    Fluid(Vec2,Vec<f32>),//min_size,vector of column width
}

pub struct GridLayout {
    ctx: Context,
    style: std::sync::Arc<Style>,
    id: Id,

    /// First frame (no previous know state).
    is_first_frame: bool,

    /// State previous frame (if any).
    /// This can be used to predict future sizes of cells.
    prev_state: State,

    /// State accumulated during the current frame.
    curr_state: State,
    initial_available: Rect,

    // Size:
    spacing: Vec2,
    size: GridLayoutSize,

    // Cursor:
    col: usize,
    row: usize,
}

impl GridLayout {
    pub fn new(ui: &Ui, id: Id, prev_state: Option<State>,grid_size: GridLayoutSize,spacing: Vec2,row:usize) -> Self {
        let is_first_frame = prev_state.is_none();
        let prev_state = prev_state.unwrap_or_default();

        // TODO(emilk): respect current layout

        let initial_available = ui.placer().max_rect().intersect(ui.cursor());
        debug_assert!(
            initial_available.min.x.is_finite(),
            "Grid not yet available for right-to-left layouts"
        );

        ui.ctx().check_for_id_clash(id, initial_available, "Grid");

        Self {
            ctx: ui.ctx().clone(),
            style: ui.style().clone(),
            id,
            is_first_frame,
            prev_state,
            curr_state: State::default(),
            initial_available,

            spacing,
            size: grid_size,

            col: 0,
            row,
        }
    }

    pub fn collapsing_size(min_size: Vec2,max_width: f32) -> GridLayoutSize {
        GridLayoutSize::Collapsing(min_size,max_width)
    }

    pub fn fluid_size(min_size: Vec2,cells_size: Vec<f32>) -> GridLayoutSize {
        GridLayoutSize::Fluid(min_size,cells_size)
    }
}

impl GridLayout {
    pub fn is_first_column(&self) -> bool {
        self.col == 0
    }
    pub fn is_last_column(&self) -> bool {
        let len = self.prev_state.col_widths.len();
        if len > 0 {
            self.col == len - 1
        } else {
            false
        }
    }
    pub fn is_first_row(&self) -> bool {
        self.row == 0
    }
    pub fn is_last_row(&self) -> bool {
        let len = self.prev_state.row_heights.len();
        if len > 0 {
            self.row == len - 1
        } else {
            false
        }
    }
    pub fn get_row_rect(&self,ui: &Ui) -> Option<Rect> {
        let Some(height) = self.prev_state.row_height(self.row) else {
            return None;
        };
        let size = Vec2::new(self.prev_state.full_width(self.spacing.x), height);
        Some(Rect::from_min_size(ui.cursor().min, size))
    }
}

impl GridLayout {
    fn prev_col_width(&self, col: usize) -> f32 {
        let col_width= match &self.size {
            GridLayoutSize::Collapsing(min_size, _) => {
                min_size.x
            }
            GridLayoutSize::Fluid(min_size,v) => {
                if col >= v.len() {
                    0.
                } else {
                    (v[col] as f32).max(min_size.x)
                }
            }
        };
        self.prev_state
            .col_width(col)
            .unwrap_or(col_width)
    }

    fn prev_row_height(&self, row: usize) -> f32 {
        let col_height= match &self.size {
            GridLayoutSize::Collapsing(min_size, _) => {
                min_size.y
            }
            GridLayoutSize::Fluid(min_size,v) => {
                min_size.y
            }
        };
        self.prev_state
            .row_height(row)
            .unwrap_or(col_height)
    }

    pub(crate) fn wrap_text(&self) -> bool {
        match self.size {
            GridLayoutSize::Collapsing(_, max_width) => {
                max_width.is_finite()
            }
            GridLayoutSize::Fluid(_, _) => {
                true
            }
        }
    }

    pub(crate) fn available_rect(&self, region: &Region) -> Rect {

        let width = match &self.size {
            GridLayoutSize::Collapsing(min_size, max_width) => {
                if max_width.is_finite() {
                    *max_width
                } else {
                    self.prev_state
                        .col_width(self.col)
                        .or_else(|| self.curr_state.col_width(self.col))
                        .unwrap_or(min_size.x)
                }
            }
            GridLayoutSize::Fluid(min_size, v) => {
                if self.col >= v.len() {
                    0.
                } else {
                    v[self.col]
                }
            }
        };
        // If something above was wider, we can be wider:
        let width = width.max(self.curr_state.col_width(self.col).unwrap_or(0.0));

        let available = region.max_rect.intersect(region.cursor);


        let mut height = region.max_rect.max.y - available.top();
        height = match &self.size {
            GridLayoutSize::Collapsing(min, _) => {
                height.at_least(min.y)
            }
            GridLayoutSize::Fluid(min, _) => {
                height.at_least(min.y)
            }
        };
        height = height.max(
            self.prev_state
                .row_height(self.row)
                .unwrap_or(0.)
        );

        Rect::from_min_size(available.min, vec2(width, height))
    }

    pub(crate) fn next_cell(&self, cursor: Rect, child_size: Vec2) -> Rect {
        let width = self.prev_state.col_width(self.col).unwrap_or(0.0);
        let height = self.prev_row_height(self.row);
        let size = child_size.max(vec2(width, height));
        Rect::from_min_size(cursor.min, size).round_ui()
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn align_size_within_rect(&self, size: Vec2, frame: Rect) -> Rect {
        // TODO(emilk): allow this alignment to be customized
        Align2::LEFT_CENTER
            .align_size_within_rect(size, frame)
            .round_ui()
    }

    pub(crate) fn justify_and_align(&self, frame: Rect, size: Vec2) -> Rect {
        self.align_size_within_rect(size, frame)
    }

    pub(crate) fn advance(&mut self, cursor: &mut Rect, _frame_rect: Rect, widget_rect: Rect) {
        #[cfg(debug_assertions)]
        {
            let debug_expand_width = self.style.debug.show_expand_width;
            let debug_expand_height = self.style.debug.show_expand_height;
            if debug_expand_width || debug_expand_height {
                let rect = widget_rect;
                let too_wide = rect.width() > self.prev_col_width(self.col);
                let too_high = rect.height() > self.prev_row_height(self.row);

                if (debug_expand_width && too_wide) || (debug_expand_height && too_high) {
                    let painter = self.ctx.debug_painter();
                    painter.rect_stroke(
                        rect,
                        0.0,
                        (1.0, Color32::LIGHT_BLUE),
                        crate::StrokeKind::Inside,
                    );

                    let stroke = Stroke::new(2.5, Color32::from_rgb(200, 0, 0));
                    let paint_line_seg = |a, b| painter.line_segment([a, b], stroke);

                    if debug_expand_width && too_wide {
                        paint_line_seg(rect.left_top(), rect.left_bottom());
                        paint_line_seg(rect.left_center(), rect.right_center());
                        paint_line_seg(rect.right_top(), rect.right_bottom());
                    }
                }
            }
        }
        let min_size = self.min_size();
        match &self.size {
            GridLayoutSize::Collapsing(_, _) => {
                self.curr_state
                    .set_min_col_width(self.col, widget_rect.width().max(min_size.x));
            }
            GridLayoutSize::Fluid(min, v) => {
                if self.col >= v.len() {
                    self.curr_state
                        .set_min_col_width(self.col, widget_rect.width().max(min_size.x));
                } else {
                    self.curr_state
                        .set_min_col_width(self.col, widget_rect.width().max(min_size.x).max(v[self.col]));
                }
            }
        }
        self.curr_state
            .set_min_row_height(self.row, widget_rect.height().max(min_size.y));

        cursor.min.x += self.prev_col_width(self.col) + self.spacing.x;
        self.col += 1;
    }

    fn min_size(&self) -> Vec2 {
        match self.size {
            GridLayoutSize::Collapsing(min, _) => {min}
            GridLayoutSize::Fluid(min, _) => {min}
        }
    }

    pub(crate) fn end_row(&mut self, cursor: &mut Rect, painter: &Painter) {
        cursor.min.x = self.initial_available.min.x;
        cursor.min.y += self.spacing.y;
        cursor.min.y += self
            .curr_state
            .row_height(self.row)
            .unwrap_or(self.min_size().y);
        cursor.max.y = cursor.min.y + self
            .curr_state
            .row_height(self.row)
            .unwrap_or(self.min_size().y);
        self.col = 0;
        self.row += 1;
    }

    pub(crate) fn save(&self) {
        // We need to always save state on the first frame, otherwise request_discard
        // would be called repeatedly (see #5132)
        if self.curr_state != self.prev_state || self.is_first_frame {
            self.curr_state.clone().store(&self.ctx, self.id);
            self.ctx.request_repaint();
        }
    }
}

// ----------------------------------------------------------------------------

/// A simple grid layout.
///
/// The cells are always laid out left to right, top-down.
/// The contents of each cell will be aligned to the left and center.
///
/// If you want to add multiple widgets to a cell you need to group them with
/// [`Ui::horizontal`], [`Ui::vertical`] etc.
///
/// ```
/// # use egui::Grid;
/// egui::__run_test_ui(|ui| {
/// egui::Grid::fluid("some_unique_id").show(ui, |ui| {
///     ui.label("First row, first column");
///     ui.label("First row, second column");
///     ui.end_row();
///
///     ui.label("Second row, first column");
///     ui.label("Second row, second column");
///     ui.label("Second row, third column");
///     ui.end_row();
///
///     ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
///     ui.label("Third row, second column");
///     ui.end_row();
/// });
/// # });
/// ```
///
struct BoundedUsize(usize);
impl BoundedUsize {
    pub fn new(value: usize) -> Self {
        if (0..=100).contains(&value) {
            BoundedUsize(value)
        } else {
            BoundedUsize(100)
        }
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl Deref for BoundedUsize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 实现与 usize 的加法：返回 usize
impl Add<usize> for BoundedUsize {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        self.0 + rhs
    }
}

// 实现与 f32 的乘法：返回 f32
impl Mul<f32> for BoundedUsize {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.0 as f32) * rhs
    }
}

enum GridSizeFluid {
    Fixed(usize), //fix width
    Percent(BoundedUsize),//percent of ui available width: usize of [0 ~ 100]
    Remainder, //Remainder of ui available width
}
pub enum GridSize {
    Collapsing(Option<usize>,Option<usize>,f32),//(min_size_x,min_size_y,max_width)
    Fluid(f32,Vec<GridSizeFluid>),//min_height,vector of column width
}

impl GridSize {
    pub fn get_min_size(&self,ui:& Ui) -> Vec2 {
        match self {
            GridSize::Collapsing(min_x, min_y, _) => {
                vec2(
                    min_x.map(|x| x as f32).unwrap_or_else(|| ui.spacing().interact_size.x),
                    min_y.map(|x| x as f32).unwrap_or_else(|| ui.spacing().interact_size.y)
                )
            }
            GridSize::Fluid(miny, _) => {
                let min = ui.spacing().interact_size;
                vec2(min.x, *miny)
            }
        }
    }
    pub fn to_grid_layout_size(self,ui:& Ui) -> GridLayoutSize {
        let min_size = self.get_min_size(ui);
        match self {
            GridSize::Collapsing(min_x, min_y, max_width) => {
                GridLayoutSize::Collapsing(min_size,max_width)
            }
            GridSize::Fluid(_,v) => {
                GridLayoutSize::Fluid(min_size,get_fluid_column_widths(ui,v))
            }

        }
    }

    pub fn collapse_size(min_w:Option<usize>,min_h:Option<usize>,max_w:f32) -> Self {
        Self::Collapsing(min_w,min_h,max_w)
    }

    pub fn fluid_size(min_h:f32) -> Self {
        Self::Fluid(min_h,vec![])
    }

    pub fn add_fixed(mut self, fixed_size:usize) -> Self {
        match self {
            GridSize::Collapsing(_,_,_) => {
                self
            }
            GridSize::Fluid(min, ref mut v) => {
                v.push(GridSizeFluid::Fixed(fixed_size));
                self
            }
        }
    }

    pub fn add_percent(mut self, percent: usize) -> Self {
        match self {
            GridSize::Collapsing(_,_,_) => {
                self
            }
            GridSize::Fluid(_, ref mut v) => {
                v.push(GridSizeFluid::Percent(BoundedUsize::new(percent)));
                self
            }
        }
    }

    pub fn add_remainder(mut self) -> Self {
        match self {
            GridSize::Collapsing(_,_,_) => {
                self
            }
            GridSize::Fluid(_, ref mut v) => {
                v.push(GridSizeFluid::Remainder);
                self
            }
        }
    }
}
#[must_use = "You should call .show()"]
pub struct GridCollapsed {
    id_salt: Id,
    min_size_x: Option<usize>,
    min_size_y: Option<usize>,
    max_width: Option<usize>,
    spacing: Option<(usize, usize)>,
    start_row: usize,
}

impl GridCollapsed {
    /// Set minimum width of each column.
    /// Default: [`crate::style::Spacing::interact_size`]`.x`.
    #[inline]
    pub fn min_col_width(mut self, min_col_width: usize) -> Self {
        self.min_size_x = Some(min_col_width);
        self
    }

    /// Set minimum height of each row.
    /// Default: [`crate::style::Spacing::interact_size`]`.y`.
    #[inline]
    pub fn min_row_height(mut self, min_row_height: usize) -> Self {
        self.min_size_y = Some(min_row_height);
        self
    }

    /// Set soft maximum width (wrapping width) of each column.
    #[inline]
    pub fn max_col_width(mut self, max_col_width: usize) -> Self {
        self.max_width = Some(max_col_width);
        self
    }

    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id_salt: Id::new(id_salt),
            min_size_x: None,
            min_size_y: None,
            max_width: None,
            spacing: None,
            start_row: 0,
        }
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let Self {
            id_salt, min_size_x, min_size_y, max_width, spacing, start_row
        } = self;
        let grid_size = GridSize::Collapsing(min_size_x,min_size_y,max_width.map(|u|u as f32).unwrap_or(f32::INFINITY));
        let grid = Grid {
            id_salt,
            size: grid_size,
            spacing: match spacing {
                None => {None}
                Some(s) => {Some(Vec2::new(s.0 as f32,s.1 as f32))}
            },
            start_row,
        };
        grid.show(ui, add_contents)
    }
}
#[must_use = "You should call .show()"]
pub struct GridFluid {
    id_salt: Id,
    columns: Vec<GridSizeFluid>,
    spacing: Option<Vec2>,
    min_height: Option<f32>,
    start_row: usize,
}

impl GridFluid {
    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id_salt: Id::new(id_salt),
            columns: Vec::new(),
            spacing: None,
            start_row: 0,
            min_height: None,
        }
    }

    pub fn fixed(mut self,column_width:usize) -> Self {
        self.columns.push(GridSizeFluid::Fixed(column_width));
        self
    }

    pub fn percent(mut self,column_width:usize) -> Self {
        self.columns.push(GridSizeFluid::Percent(BoundedUsize::new(column_width)));
        self
    }

    pub fn remainder(mut self) -> Self {
        self.columns.push(GridSizeFluid::Remainder);
        self
    }

    /// Set spacing between columns/rows.
    /// Default: [`crate::style::Spacing::item_spacing`].
    #[inline]
    pub fn spacing(mut self, spacing: impl Into<Vec2>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn min_row_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let Self { id_salt, columns, spacing, min_height, start_row,  } = self;
        let grid_size = GridSize::Fluid(min_height.unwrap_or(ui.spacing().interact_size.y),columns);
        let grid = Grid {
            id_salt,
            size: grid_size,
            spacing,
            start_row,
        };
        grid.show(ui, add_contents)
    }
}

#[must_use = "You should call .show()"]
pub struct Grid {
    id_salt: Id,
    size: GridSize,
    spacing: Option<Vec2>,
    start_row: usize,
}

impl Grid {
    // TODO Create
    // new struct GridRow and GridColumn
    pub fn row<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let res = add_contents(ui);
        ui.end_row();
        res
    }

    /// Create a new [`Grid`] with a locally unique identifier.
    pub fn new(id_salt: impl std::hash::Hash,grid_size:GridSize) -> Self {
        Self {
            id_salt: Id::new(id_salt),
            size: grid_size,
            spacing: None,
            start_row: 0,
        }
    }

    pub fn collapsed(id_salt: impl std::hash::Hash) -> GridCollapsed {
        GridCollapsed::new(id_salt)
    }

    pub fn fluid(id_salt: impl std::hash::Hash) -> GridFluid {
        GridFluid::new(id_salt)
    }

    /// Set spacing between columns/rows.
    /// Default: [`crate::style::Spacing::item_spacing`].
    #[inline]
    pub fn spacing(mut self, spacing: impl Into<Vec2>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    /// Change which row number the grid starts on.
    /// This can be useful when you have a large [`crate::Grid`] inside of [`crate::ScrollArea::show_rows`].
    #[inline]
    pub fn start_row(mut self, start_row: usize) -> Self {
        self.start_row = start_row;
        self
    }
}

impl Grid {
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        self.show_dyn(ui, Box::new(add_contents))
    }

    fn show_dyn<'c, R>(
        self,
        ui: &mut Ui,
        add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<R> {
        let Self {
            id_salt,
            size,
            spacing,
            start_row,
        } = self;
        let spacing = spacing.unwrap_or_else(|| ui.spacing().item_spacing);

        let id = ui.make_persistent_id(id_salt);
        let prev_state = State::load(ui.ctx(), id);

        // Each grid cell is aligned LEFT_CENTER.
        // If somebody wants to wrap more things inside a cell,
        // then we should pick a default layout that matches that alignment,
        // which we do here:
        let max_rect = ui.cursor().intersect(ui.max_rect());

        let mut ui_builder = UiBuilder::new().max_rect(max_rect);
        if prev_state.is_none() {
            // The initial frame will be glitchy, because we don't know the sizes of things to come.

            if ui.is_visible() {
                // Try to cover up the glitchy initial frame:
                ui.ctx().request_discard("new Grid");
            }

            // Hide the ui this frame, and make things as narrow as possible:
            ui_builder = ui_builder.sizing_pass().invisible();
        }

        ui.allocate_new_ui(ui_builder, |ui| {
            ui.horizontal(|ui| {
                let grid = GridLayout::new(ui, id, prev_state,size.to_grid_layout_size(ui),spacing,start_row);

                ui.set_grid(grid);
                let r = add_contents(ui);
                ui.save_grid();
                r
            })
            .inner
        })
    }
}

fn striped_row_color(row: usize, style: &Style) -> Option<Color32> {
    if row % 2 == 1 {
        return Some(style.visuals.faint_bg_color);
    }
    None
}

fn get_fluid_column_widths(ui: & Ui,grid_columns: Vec<GridSizeFluid>) -> Vec<(f32)> {
    let full_size = ui.available_size().x;
    let mut remainer_width = full_size;
    let min_width = ui.spacing().interact_size.x;
    let mut res = vec![];
    for column in grid_columns {
        let mut c_width = match column {
            GridSizeFluid::Fixed(column_width) => {
                column_width as f32
            },
            GridSizeFluid::Percent(percent) => {
                (percent * 0.01 * full_size).floor()
            },
            GridSizeFluid::Remainder => {
                remainer_width
            },
        };
        c_width = c_width.at_least(min_width).at_most(remainer_width);
        remainer_width -= c_width;
        if remainer_width < min_width {
            res.push(c_width + remainer_width);
            break
        } else {
            res.push(c_width);
        }
    }
    res
}
