use std::thread;

use druid::{
    widget::prelude::*, AppDelegate, AppLauncher, Data, ExtEventSink, Handled, SingleUse, Target,
};
use flume::{Receiver, Sender};
use futures::{future, prelude::*};
use tokio::runtime;

use super::{
    future_widget::{FutureRequest, FutureResponse, FUTURE_ASYNC_RESPONSE, FUTURE_SPAWN_ASYNC},
    stream_widget::{StreamRequest, StreamResponse, STREAM_ASYNC_RESPONSE, STREAM_SPAWN_ASYNC},
};
pub struct AsyncDelegate<T: Data + 'static> {
    future_tx: Sender<FutureRequest>,
    stream_tx: Sender<StreamRequest>,
    inner_delegate: Option<Box<dyn AppDelegate<T>>>,
}

fn create_channel<T: Data + 'static, U: Send + 'static>(
    launcher: &AppLauncher<T>,
    thread_handler: impl Fn(ExtEventSink, Receiver<U>) + Send + 'static,
) -> Sender<U> {
    let sink = launcher.get_external_handle();
    let (tx, rx) = flume::unbounded();
    thread::spawn(move || {
        thread_handler(sink, rx);
    });
    tx
}

impl<T: Data + 'static> AsyncDelegate<T> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(launcher: AppLauncher<T>) -> AppLauncher<T> {
        let future_tx = create_channel(&launcher, future_handle_thread);
        let stream_tx = create_channel(&launcher, stream_handle_thread);

        launcher.delegate(Self {
            future_tx,
            stream_tx,
            inner_delegate: None,
        })
    }

    pub fn with_delegate(
        launcher: AppLauncher<T>,
        delegate: impl AppDelegate<T> + 'static,
    ) -> AppLauncher<T> {
        let future_tx = create_channel(&launcher, future_handle_thread);
        let stream_tx = create_channel(&launcher, stream_handle_thread);

        launcher.delegate(Self {
            future_tx,
            stream_tx,
            inner_delegate: Some(Box::new(delegate)),
        })
    }
}

impl<T: Data + 'static> AppDelegate<T> for AsyncDelegate<T> {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut T,
        env: &Env,
    ) -> Handled {
        if let Some(req) = cmd.get(FUTURE_SPAWN_ASYNC) {
            let req = req
                .take()
                .expect("Someone stole our FUTURE_SPAWN_ASYNC command.");
            self.future_tx.send(req).unwrap();
            Handled::Yes
        } else if let Some(req) = cmd.get(STREAM_SPAWN_ASYNC) {
            let req = req
                .take()
                .expect("Someone stole our STREAM_SPAWN_ASYNC command.");
            self.stream_tx.send(req).unwrap();
            Handled::Yes
        } else if let Some(inner_delegate) = self.inner_delegate.as_mut() {
            inner_delegate.command(ctx, target, cmd, data, env)
        } else {
            Handled::No
        }
    }

    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        window_id: druid::WindowId,
        event: Event,
        data: &mut T,
        env: &Env,
    ) -> Option<Event> {
        self.inner_delegate
            .as_mut()?
            .event(ctx, window_id, event, data, env)
    }

    fn window_added(
        &mut self,
        id: druid::WindowId,
        data: &mut T,
        env: &Env,
        ctx: &mut druid::DelegateCtx,
    ) {
        if let Some(inner_delegate) = self.inner_delegate.as_mut() {
            inner_delegate.window_added(id, data, env, ctx)
        }
    }

    fn window_removed(
        &mut self,
        id: druid::WindowId,
        data: &mut T,
        env: &Env,
        ctx: &mut druid::DelegateCtx,
    ) {
        if let Some(inner_delegate) = self.inner_delegate.as_mut() {
            inner_delegate.window_removed(id, data, env, ctx)
        }
    }
}

// TODO: make this work with other runtimes
fn future_handle_thread(sink: ExtEventSink, rx: Receiver<FutureRequest>) {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let rx = rx.stream();
        rx.for_each(|req| {
            let sink = sink.clone();
            rt.spawn(async move {
                let res = req.future.await;
                let res = FutureResponse { value: res };
                let sender = req.sender;

                sink.submit_command(
                    FUTURE_ASYNC_RESPONSE,
                    SingleUse::new(res),
                    Target::Widget(sender),
                )
                .unwrap();
            });
            future::ready(())
        })
        .await;
    });
}

fn stream_handle_thread(sink: ExtEventSink, rx: Receiver<StreamRequest>) {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let rx = rx.stream();
        rx.for_each(|mut req| {
            let sink = sink.clone();
            rt.spawn(async move {
                while let Some(res) = req.stream.next().await {
                    let res = StreamResponse { value: res };
                    let sender = req.sender;

                    sink.submit_command(
                        STREAM_ASYNC_RESPONSE,
                        SingleUse::new(res),
                        Target::Widget(sender),
                    )
                    .unwrap();
                }
            });
            future::ready(())
        })
        .await;
    });
}
