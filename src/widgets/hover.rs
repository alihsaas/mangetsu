use druid::{widget::prelude::*, Color, Data, KeyOrValue, Widget};

pub struct Hover<T> {
    inner: Box<dyn Widget<T>>,
    hover_color: KeyOrValue<Color>,
}

impl<T: Data> Hover<T> {
    pub fn new(inner: impl Widget<T> + 'static, hover_color: impl Into<KeyOrValue<Color>>) -> Self {
        Self {
            inner: Box::new(inner),
            hover_color: hover_color.into(),
        }
    }
}

impl<T: Data> Widget<T> for Hover<T> {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        let background = ctx.size().to_rect();
        if ctx.is_hot() {
            ctx.fill(background, &self.hover_color.resolve(env));
        }
        self.inner.paint(ctx, data, env)
    }
}
