use skia_safe::{self as skia, Matrix, Point};

use std::collections::HashMap;
use uuid::Uuid;

use crate::render::BlendMode;

mod blurs;
mod bools;
mod fills;
mod frames;
mod groups;
mod layouts;
mod modifiers;
mod paths;
mod rects;
mod shadows;
mod strokes;
mod svgraw;
mod transform;

pub use blurs::*;
pub use bools::*;
pub use fills::*;
pub use frames::*;
pub use groups::*;
pub use layouts::*;
pub use modifiers::*;
pub use paths::*;
pub use rects::*;
pub use shadows::*;
pub use strokes::*;
pub use svgraw::*;
pub use transform::*;

use crate::math::Bounds;

pub type CornerRadius = Point;
pub type Corners = [CornerRadius; 4];

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Frame(Frame),
    Group(Group),
    Bool(Bool),
    Rect(Rect),
    Path(Path),
    Text,
    Circle,
    SVGRaw(SVGRaw)
}

impl Type {
    pub fn from(value: u8) -> Self {
        match value {
            0 => Type::Frame(Frame::default()),
            1 => Type::Group(Group::default()),
            2 => Type::Bool(Bool::default()),
            3 => Type::Rect(Rect::default()),
            4 => Type::Path(Path::default()),
            5 => Type::Text,
            6 => Type::Circle,
            7 => Type::SVGRaw(SVGRaw::default()),
            _ => Type::Rect(Rect::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ConstraintH {
    Left,
    Right,
    LeftRight,
    Center,
    Scale,
}

impl ConstraintH {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            2 => Some(Self::LeftRight),
            3 => Some(Self::Center),
            4 => Some(Self::Scale),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ConstraintV {
    Top,
    Bottom,
    TopBottom,
    Center,
    Scale,
}

impl ConstraintV {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Top),
            1 => Some(Self::Bottom),
            2 => Some(Self::TopBottom),
            3 => Some(Self::Center),
            4 => Some(Self::Scale),
            _ => None,
        }
    }
}

pub type Color = skia::Color;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Shape {
    pub id: Uuid,
    pub shape_type: Type,
    pub children: Vec<Uuid>,
    pub selrect: skia::Rect,
    pub transform: Matrix,
    pub rotation: f32,
    pub constraint_h: Option<ConstraintH>,
    pub constraint_v: Option<ConstraintV>,
    pub clip_content: bool,
    pub fills: Vec<Fill>,
    pub strokes: Vec<Stroke>,
    pub blend_mode: BlendMode,
    pub blur: Blur,
    pub opacity: f32,
    pub hidden: bool,
    pub svg: Option<skia::svg::Dom>,
    pub svg_attrs: HashMap<String, String>,
    pub shadows: Vec<Shadow>,
    pub layout_item: Option<LayoutItem>,
}

impl Shape {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            shape_type: Type::Rect(Rect::default()),
            children: Vec::<Uuid>::new(),
            selrect: skia::Rect::new_empty(),
            transform: Matrix::default(),
            rotation: 0.,
            constraint_h: None,
            constraint_v: None,
            clip_content: true,
            fills: vec![],
            strokes: vec![],
            blend_mode: BlendMode::default(),
            opacity: 1.,
            hidden: false,
            blur: Blur::default(),
            svg: None,
            svg_attrs: HashMap::new(),
            shadows: vec![],
            layout_item: None
        }
    }

    pub fn set_shape_type(&mut self, shape_type: Type) {
        self.shape_type = shape_type;
    }

    pub fn is_frame(&self) -> bool {
        matches!(self.shape_type, Type::Frame(_))
    }

    pub fn set_selrect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.selrect.set_ltrb(left, top, right, bottom);
    }

    pub fn set_masked(&mut self, masked: bool) {
        match &mut self.shape_type {
            Type::Group(data) => {
                data.masked = masked;
            }
            _ => {}
        }
    }

    pub fn set_clip(&mut self, value: bool) {
        self.clip_content = value;
    }

    pub fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }

    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        self.transform = Matrix::new_all(a, c, e, b, d, f, 0.0, 0.0, 1.0);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity;
    }

    pub fn set_constraint_h(&mut self, constraint: Option<ConstraintH>) {
        self.constraint_h = constraint;
    }

    pub fn constraint_h(&self, default: ConstraintH) -> ConstraintH {
        self.constraint_h.clone().unwrap_or(default)
    }

    pub fn set_constraint_v(&mut self, constraint: Option<ConstraintV>) {
        self.constraint_v = constraint;
    }

    pub fn constraint_v(&self, default: ConstraintV) -> ConstraintV {
        self.constraint_v.clone().unwrap_or(default)
    }

    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = value;
    }

    pub fn set_blur(&mut self, blur_type: u8, hidden: bool, value: f32) {
        self.blur = Blur::new(blur_type, hidden, value);
    }

    pub fn add_child(&mut self, id: Uuid) {
        self.children.push(id);
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    pub fn fills(&self) -> std::slice::Iter<Fill> {
        self.fills.iter()
    }

    pub fn add_fill(&mut self, f: Fill) {
        self.fills.push(f)
    }

    pub fn clear_fills(&mut self) {
        self.fills.clear();
    }

    pub fn add_fill_gradient_stops(&mut self, buffer: Vec<RawStopData>) -> Result<(), String> {
        let fill = self.fills.last_mut().ok_or("Shape has no fills")?;
        let gradient = match fill {
            Fill::LinearGradient(g) => Ok(g),
            Fill::RadialGradient(g) => Ok(g),
            _ => Err("Active fill is not a gradient"),
        }?;

        for stop in buffer.into_iter() {
            gradient.add_stop(stop.color(), stop.offset());
        }

        Ok(())
    }

    pub fn strokes(&self) -> std::slice::Iter<Stroke> {
        self.strokes.iter()
    }

    pub fn add_stroke(&mut self, s: Stroke) {
        self.strokes.push(s)
    }

    pub fn set_stroke_fill(&mut self, f: Fill) -> Result<(), String> {
        let stroke = self.strokes.last_mut().ok_or("Shape has no strokes")?;
        stroke.fill = f;
        Ok(())
    }

    pub fn add_stroke_gradient_stops(&mut self, buffer: Vec<RawStopData>) -> Result<(), String> {
        let stroke = self.strokes.last_mut().ok_or("Shape has no strokes")?;
        let fill = &mut stroke.fill;
        let gradient = match fill {
            Fill::LinearGradient(g) => Ok(g),
            Fill::RadialGradient(g) => Ok(g),
            _ => Err("Active stroke is not a gradient"),
        }?;

        for stop in buffer.into_iter() {
            gradient.add_stop(stop.color(), stop.offset());
        }

        Ok(())
    }

    pub fn clear_strokes(&mut self) {
        self.strokes.clear();
    }

    pub fn set_path_segments(&mut self, buffer: Vec<RawPathData>) -> Result<(), String> {
        let path = Path::try_from(buffer)?;

        match &mut self.shape_type {
            Type::Bool(Bool { bool_type, .. }) => {
                self.shape_type = Type::Bool(Bool {
                    bool_type: *bool_type,
                    path
                });
            },
            Type::Path(_) => {
                self.shape_type = Type::Path(path);
            },
            _ => {}
        };
        Ok(())
    }

    pub fn set_path_attr(&mut self, name: String, value: String) {
        match self.shape_type {
            Type::Path(_) => {
                self.set_svg_attr(name, value);
            }
            _ => unreachable!("This shape should have path attrs"),
        };
    }

    pub fn set_svg_raw_content(&mut self, content: String) -> Result<(), String> {
        self.shape_type = Type::SVGRaw(SVGRaw::from_content(content));
        Ok(())
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode;
    }

    pub fn set_bool_type(&mut self, bool_type: BoolType) {
        self.shape_type = match &self.shape_type {
            Type::Bool(Bool { path, .. }) => {
                Type::Bool(Bool { bool_type, path: path.clone() })
            },
            _ => Type::Bool(Bool {bool_type, path: Path::default()}),
        };
    }

    pub fn set_corners(&mut self, raw_corners: (f32, f32, f32, f32)) {
        match &mut self.shape_type {
            Type::Rect(data) => {
                data.corners = make_corners(raw_corners);
            }
            Type::Frame(data) => {
                data.corners = make_corners(raw_corners);
            }
            _ => {}
        }
    }

    pub fn set_svg(&mut self, svg: skia::svg::Dom) {
        self.svg = Some(svg);
    }

    pub fn set_svg_attr(&mut self, name: String, value: String) {
        self.svg_attrs.insert(name, value);
    }

    pub fn blend_mode(&self) -> crate::render::BlendMode {
        self.blend_mode
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    // TODO: Maybe store this inside the shape
    pub fn bounds(&self) -> Bounds {
        let mut bounds = Bounds::new(
            Point::new(self.selrect.x(), self.selrect.y()),
            Point::new(self.selrect.x() + self.selrect.width(), self.selrect.y()),
            Point::new(
                self.selrect.x() + self.selrect.width(),
                self.selrect.y() + self.selrect.height(),
            ),
            Point::new(self.selrect.x(), self.selrect.y() + self.selrect.height()),
        );

        let center = self.center();
        let mut matrix = self.transform.clone();
        matrix.post_translate(center);
        matrix.pre_translate(-center);

        bounds.transform_mut(&matrix);

        bounds
    }

    pub fn selrect(&self) -> skia::Rect {
        self.selrect
    }

    pub fn center(&self) -> Point {
        self.selrect.center()
    }

    pub fn clip(&self) -> bool {
        self.clip_content
    }

    pub fn mask_id(&self) -> Option<&Uuid> {
        self.children.first()
    }

    pub fn children_ids(&self) -> Vec<Uuid> {
        if let Type::Bool(_) = self.shape_type {
            vec![]
        } else if let Type::Group(group) = self.shape_type {
            if group.masked {
                self.children[1..self.children.len()].to_vec()
            } else {
                self.children.clone()
            }
        } else {
            self.children.clone()
        }
    }

    pub fn image_filter(&self, scale: f32) -> Option<skia::ImageFilter> {
        if !self.blur.hidden {
            match self.blur.blur_type {
                BlurType::None => None,
                BlurType::Layer => skia::image_filters::blur(
                    (self.blur.value * scale, self.blur.value * scale),
                    None,
                    None,
                    None,
                ),
            }
        } else {
            None
        }
    }

    pub fn is_recursive(&self) -> bool {
        !matches!(self.shape_type, Type::SVGRaw(_))
    }

    pub fn add_shadow(&mut self, shadow: Shadow) {
        self.shadows.push(shadow);
    }

    pub fn clear_shadows(&mut self) {
        self.shadows.clear();
    }

    pub fn drop_shadows(&self) -> impl DoubleEndedIterator<Item = &Shadow> {
        self.shadows
            .iter()
            .filter(|shadow| shadow.style() == ShadowStyle::Drop)
    }

    pub fn inner_shadows(&self) -> impl DoubleEndedIterator<Item = &Shadow> {
        self.shadows
            .iter()
            .filter(|shadow| shadow.style() == ShadowStyle::Inner)
    }

    pub fn to_path_transform(&self) -> Option<Matrix> {
        match self.shape_type {
            Type::Path(_) | Type::Bool(_) => {
                let center = self.center();
                let mut matrix = Matrix::new_identity();
                matrix.pre_translate(center);
                matrix.pre_concat(&self.transform.invert()?);
                matrix.pre_translate(-center);
                Some(matrix)
            }
            _ => None,
        }
    }

    fn transform_selrect(&mut self, transform: &Matrix) {
        let mut center = self.selrect.center();
        center = transform.map_point(center);

        let bounds = self.bounds().transform(&transform);
        self.transform = bounds.transform_matrix().unwrap_or(Matrix::default());

        let width = bounds.width();
        let height = bounds.height();

        self.selrect = skia::Rect::from_xywh(
            center.x - width / 2.0,
            center.y - height / 2.0,
            width,
            height,
        );
    }

    pub fn apply_transform(&mut self, transform: &Matrix) {
        self.transform_selrect(&transform);
        match &mut self.shape_type {
            Type::Path(path) => {
                path.transform(&transform);
            }
            Type::Bool(Bool { path, .. }) => {
                path.transform(&transform);
            }
            _ => {}
        }
    }
}

fn make_corners(raw_corners: (f32, f32, f32, f32)) -> Option<Corners> {
    let (r1, r2, r3, r4) = raw_corners;
    let are_straight_corners = r1.abs() <= f32::EPSILON
        && r2.abs() <= f32::EPSILON
        && r3.abs() <= f32::EPSILON
        && r4.abs() <= f32::EPSILON;

    if are_straight_corners {
        None
    } else {
        Some([
            (r1, r1).into(),
            (r2, r2).into(),
            (r3, r3).into(),
            (r4, r4).into(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_shape() -> Shape {
        Shape::new(Uuid::nil())
    }

    #[test]
    fn add_fill_pushes_a_new_fill() {
        let mut shape = any_shape();
        assert_eq!(shape.fills.len(), 0);

        shape.add_fill(Fill::Solid(Color::TRANSPARENT));
        assert_eq!(shape.fills.get(0), Some(&Fill::Solid(Color::TRANSPARENT)))
    }

    #[test]
    fn test_set_corners() {
        let mut shape = any_shape();
        shape.set_corners((10.0, 20.0, 30.0, 40.0));
        if let Type::Rect(Rect {corners, ..}) = shape.shape_type {
            assert_eq!(corners, Some([
                Point { x: 10.0, y: 10.0 },
                Point { x: 20.0, y: 20.0 },
                Point { x: 30.0, y: 30.0 },
                Point { x: 40.0, y: 40.0 }
            ]));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_set_masked() {
        let mut shape = any_shape();
        shape.set_shape_type(Type::Group(Group { masked: false }));
        shape.set_masked(true);

        if let Type::Group(Group { masked, .. }) = shape.shape_type {
            assert_eq!(masked, true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_apply_transform() {
        let mut shape = Shape::new(Uuid::new_v4());
        shape.set_shape_type(Type::Rect(Rect::default()));
        shape.set_selrect(0.0, 10.0, 10.0, 0.0);
        shape.apply_transform(&Matrix::scale((2.0, 2.0)));

        assert_eq!(shape.selrect().width(), 20.0);
        assert_eq!(shape.selrect().height(), 20.0);
    }

}


