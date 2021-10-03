use druid::{
    im::Vector,
    widget::{prelude::*, Controller},
};

use crate::data::{cmd, AppState, MangaDetail, Nav};

pub struct NavController;

impl NavController {
    fn load_route_data(&self, ctx: &mut EventCtx, data: &mut AppState) {
        match &data.route {
            Nav::Home => {}
            Nav::Downloads => {}
            Nav::MangaPage(url) => {
                if let Some(manga) = data.manga_cache.lock().unwrap().get_mut(url) {
                    data.manga_detail = Some(MangaDetail {
                        manga: manga.to_owned(),
                        start: 1,
                        end: 1,
                        chapters: Vector::new(),
                    });
                } else {
                    ctx.submit_command(cmd::FETCH_MANGA_DETAIL.with(url.to_owned()));
                }
            }
        };
        data.mangas = Vector::new();
    }
}

impl<W> Controller<AppState, W> for NavController
where
    W: Widget<AppState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(cmd::NAVIGATE) => {
                let nav = cmd.get_unchecked(cmd::NAVIGATE);
                data.navigate(nav);
                ctx.set_handled();
                self.load_route_data(ctx, data);
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }
}
