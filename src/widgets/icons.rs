#![allow(dead_code)]

use crate::ui::theme;
use druid::{kurbo::BezPath, widget::prelude::*, Affine, Color, KeyOrValue, Size};

pub static QUIT_APP: SvgIcon = SvgIcon {
    svg_path: "M10.2 0.7 9.5 0 5.1 4.4 0.7 0 0 0.7 4.4 5.1 0 9.5 0.7 10.2 5.1 5.8 9.5 10.2 10.2 9.5 5.8 5.1Z",
    svg_size: Size::new(12., 12.),
    op: PaintOp::Fill,
};

pub static RESTORED: SvgIcon = SvgIcon {
    svg_path: "M2,0 L8,0 L8,6 M0,3 L6,3 M0,2 L6,2 L6,8 L0,8 Z",
    svg_size: Size::new(8., 8.),
    op: PaintOp::Stroke { width: 1. },
};

pub static MAXIMIZED: SvgIcon = SvgIcon {
    svg_path: "M0,1 L9,1 L9,8 L0,8 Z",
    svg_size: Size::new(9., 8.),
    op: PaintOp::Stroke { width: 1. },
};

#[derive(Copy, Clone)]
pub enum PaintOp {
    Fill,
    Stroke { width: f64 },
}

#[derive(Clone)]
pub struct SvgIcon {
    svg_path: &'static str,
    svg_size: Size,
    op: PaintOp,
}

impl SvgIcon {
    pub fn scale(&self, to_size: impl Into<Size>) -> Icon {
        let to_size = to_size.into();
        let bez_path = BezPath::from_svg(self.svg_path).expect("Failed to parse SVG");
        let scale = Affine::scale_non_uniform(
            to_size.width / self.svg_size.width,
            to_size.height / self.svg_size.height,
        );
        Icon::new(self.op, bez_path, to_size, scale)
    }
}

#[derive(Clone)]
pub struct Icon {
    op: PaintOp,
    bez_path: BezPath,
    size: Size,
    scale: Affine,
    color: KeyOrValue<Color>,
}

impl Icon {
    pub fn new(op: PaintOp, bez_path: BezPath, size: Size, scale: Affine) -> Self {
        Icon {
            op,
            bez_path,
            size,
            scale,
            color: theme::ICON_COLOR.into(),
        }
    }

    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }
}

impl<T> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let color = self.color.resolve(env);
        ctx.with_save(|ctx| {
            ctx.transform(self.scale);
            match self.op {
                PaintOp::Fill => ctx.fill(&self.bez_path, &color),
                PaintOp::Stroke { width } => ctx.stroke(&self.bez_path, &color, width),
            }
        });
    }
}
