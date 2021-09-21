mod async_delegate;
mod dynamic_sized_box;
pub mod future_widget;
mod grid;
mod hover;
pub mod icons;
mod link;
pub mod remote_image;
pub mod stream_widget;
mod theme;
mod titlebar;
mod utils;

use druid::{widget::Padding, Color, Data, Insets, KeyOrValue, Widget};

pub use async_delegate::AsyncDelegate;
pub use dynamic_sized_box::DynamicSizedBox;
pub use future_widget::FutureWidget;
pub use grid::GridView;
pub use hover::Hover;
pub use link::Link;
pub use stream_widget::StreamWidget;
pub use theme::ThemeScope;
pub use titlebar::TitleBar;
pub use utils::{Border, Clip, Logger};

pub trait MyWidgetExt<T: Data>: Widget<T> + Sized + 'static {
    fn log(self, label: &'static str) -> Logger<Self> {
        Logger::new(self).with_label(label)
    }

    fn link(self) -> Link<T> {
        Link::new(self)
    }

    fn clip<S>(self, shape: S) -> Clip<S, Self> {
        Clip::new(shape, self)
    }

    fn hover(self, hover_color: impl Into<KeyOrValue<Color>>) -> Hover<T> {
        Hover::new(self, hover_color)
    }

    fn padding_right(self, p: f64) -> Padding<T> {
        Padding::new(Insets::new(0.0, 0.0, p, 0.0), self)
    }

    /*
    fn on_debounce(
        self,
        duration: Duration,
        handler: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, OnDebounce<T>> {
        ControllerHost::new(self, OnDebounce::trailing(duration, handler))
    }

    fn on_update<F>(self, handler: F) -> ControllerHost<Self, OnUpdate<F>>
    where
        F: Fn(&mut UpdateCtx, &T, &T, &Env) + 'static,
    {
        ControllerHost::new(self, OnUpdate::new(handler))
    }
    fn on_right_click(
        self,
        func: impl Fn(&mut EventCtx, &MouseEvent, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        ControllerHost::new(self, ExClick::new(Some(MouseButton::Right), func))
    }

    fn on_command<U, F>(
        self,
        selector: Selector<U>,
        func: F,
    ) -> ControllerHost<Self, OnCommand<U, F>>
    where
        U: 'static,
        F: Fn(&mut EventCtx, &U, &mut T),
    {
        ControllerHost::new(self, OnCommand::new(selector, func))
    }

    fn on_command_async<U: Data + Send, V: Data + Send>(
        self,
        selector: Selector<U>,
        request: impl Fn(U) -> V + Sync + Send + 'static,
        preflight: impl Fn(&mut EventCtx, &mut T, U) + 'static,
        response: impl Fn(&mut EventCtx, &mut T, (U, V)) + 'static,
    ) -> OnCommandAsync<Self, T, U, V> {
        OnCommandAsync::new(
            self,
            selector,
            Box::new(preflight),
            Arc::new(request),
            Box::new(response),
        )
    }

    fn context_menu(
        self,
        func: impl Fn(&T) -> Menu<AppState> + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        self.on_right_click(move |ctx, event, data, _env| {
            ctx.show_context_menu(func(data), event.window_pos);
        })
    }
    */
}

impl<T: Data, W: Widget<T> + 'static> MyWidgetExt<T> for W {}
