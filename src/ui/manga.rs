use druid::{
    im::Vector,
    widget::{Flex, Label, Spinner, Scroll},
    Widget, WidgetExt, Color,
};

use crate::data::{AppState, Nav, cmd};
use crate::core::Manga;
use crate::widgets::{GridView, remote_image::RemoteImage};

use super::manga;

pub fn manga_widget() -> impl Widget<Manga> {
    Flex::column()
        .with_child(
            RemoteImage::new(
                Spinner::new().fix_size(20., 20.).center(),
                |data: &Manga, _| Some(data.icon_url.to_string().into()),
            )
            .fix_height(162.5),
        )
        .with_child(
            Label::raw()
                .with_text_alignment(druid::TextAlignment::Start)
                .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                .lens(Manga::title),
        )
        .fix_width(112.5)
        .background(Color::BLACK)
        .on_click(|ctx, data , _| {
            ctx.submit_command(cmd::NAVIGATE.with(Nav::MangaPage(data.clone())))
        })
}

pub fn mangas_widget() -> impl Widget<Vector<Manga>> {
    Scroll::new(GridView::new(manga::manga_widget).wrap().with_spacing(10.)).vertical()
}

pub fn manga_page_widget(manga: &Manga) -> impl Widget<AppState> {
    let manga = manga.clone();
    Flex::column()
        .with_spacer(30.)
        .with_child(
            Flex::row()
                .with_spacer(10.)
                .with_child(
                    RemoteImage::new(
                Spinner::new().fix_size(40., 40.).center(),
                move |data: &AppState, _| Some(manga.icon_url.to_string().into()),
                    ).fix_size(225., 325.)
                )
        )
}
