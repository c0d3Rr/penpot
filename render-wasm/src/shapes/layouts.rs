#![allow(dead_code)]


#[derive(Debug, Clone, PartialEq)]
pub enum Layout {
    FlexLayout(LayoutData, FlexData),
    GridLayout(LayoutData, GridData)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlignContent {
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifyItems {
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent {
}

#[derive(Debug, Clone, PartialEq)]
pub enum WrapType {
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridTrack {
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sizing {
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutData {
    pub direction: Direction,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub justify_items: JustifyItems,
    pub justify_content: JustifyContent,
    pub padding: [f32;4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlexData {
    pub row_gap: f32,
    pub column_gap: f32,
    pub wrap_type: WrapType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridData {
    pub rows: Vec<GridTrack>,
    pub columns: Vec<GridTrack>,
    // layout-grid-cells       ;; map of id->grid-cell
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutItem {
    pub margin: [f32;4],
    pub h_sizing: Sizing,
    pub v_sizing: Sizing,
    pub max_height: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub min_width: f32,
    pub absolute: bool,
    pub z_index: i32,
}
