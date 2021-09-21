use std::{any::Any, pin::Pin};

use druid::{widget::prelude::*, Data, Env, LifeCycle, Selector, SingleUse, Widget, WidgetPod};
use futures::{stream::BoxStream, Stream, StreamExt};

pub struct StreamRequest {
    pub stream: Pin<Box<dyn Stream<Item = Box<dyn Any + Send>> + Send>>,
    pub sender: WidgetId,
}

pub struct StreamResponse {
    pub value: Box<dyn Any + Send>,
}

pub const STREAM_ASYNC_RESPONSE: Selector<SingleUse<StreamResponse>> =
    Selector::new("druid-async.stream-async-response");
pub const STREAM_SPAWN_ASYNC: Selector<SingleUse<StreamRequest>> =
    Selector::new("druid-async.stream-spawn-async");

pub type AsyncGridAction<T> = Box<dyn FnOnce(&T, &Env) -> BoxStream<'static, Box<dyn Any + Send>>>;
pub type AsyncGridDone<T, U> = Box<dyn Fn(Box<U>, &mut T, &Env)>;

pub struct StreamWidget<T, U> {
    stream: Option<AsyncGridAction<T>>,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    on_done: AsyncGridDone<T, U>,
}

/// The number of elements found on the minor axis of the grid
enum MinorAxisCount {
    /// If this is wrap, the grid determines the max amount of items per
    /// minor axis. Wrap assumes the grid items are equal in size.
    Wrap,
    /// A user specified number of elements on minor axis. Can overflow
    /// the container if the count * size of grid items is larger than container
    Count(u64), // this should probably take a KeyOrValue<u64> instead
}

impl<T, U> StreamWidget<T, U> {
    pub fn new<SMaker, IStream, Done>(
        inner: impl Widget<T> + 'static,
        stream_maker: SMaker,
        on_done: Done,
    ) -> Self
    where
        U: Send + 'static,
        SMaker: FnOnce(&T, &Env) -> IStream + 'static,
        IStream: Stream<Item = U> + 'static + Send,
        Done: Fn(Box<U>, &mut T, &Env) + 'static,
    {
        Self {
            stream: Some(Box::new(move |data, env| {
                let mut fut = Box::pin(stream_maker(data, env));
                Box::pin(
                    async_stream::stream! { while let Some(res) = fut.next().await { yield Box::new(res) as _ } },
                )
            })),
            inner: WidgetPod::new(Box::new(inner)),
            on_done: Box::new(on_done),
        }
    }
}

impl<T: Data, U: 'static> Widget<T> for StreamWidget<T, U> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some(res) = cmd.get(STREAM_ASYNC_RESPONSE) {
                let res = res.take().unwrap();
                let value = res.value.downcast::<U>().unwrap();
                (self.on_done)(value, data, env);
                ctx.children_changed();
                return;
            }
            #[cfg(debug_assertions)]
            if cmd.is(STREAM_SPAWN_ASYNC) {
                // FUTURE_SPAWN_ASYNC should always be handled by the delegate
                panic!("StreamWidget used without using AsyncDelegate");
            }
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            ctx.submit_command(STREAM_SPAWN_ASYNC.with(SingleUse::new(StreamRequest {
                stream: (self.stream.take().unwrap())(data, env),
                sender: ctx.widget_id(),
            })));
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env)
    }
}
