use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, FlexParams, Label, Scroll, Spinner},
    Color, LensExt, UnitPoint, Widget, WidgetExt,
};
use futures::StreamExt;

use crate::{
    core::{error::Error, Chapter, GlobalAPI, Manga},
    data::{cmd, MangaDetail, Nav},
    widgets::{remote_image::RemoteImage, FutureWidget, GridView, Maybe},
};

use super::{chapter::chapters_widget, manga, theme};

pub fn manga_widget() -> impl Widget<Manga> {
    Flex::column()
        .with_child(
            RemoteImage::new(
                Spinner::new().fix_size(20., 20.).center(),
                |data: &Manga, _| Some(data.icon_url.clone()),
            )
            .fix_height(162.5),
        )
        .with_child(
            Label::raw()
                .with_text_alignment(druid::TextAlignment::Start)
                .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                .with_text_color(theme::TEXT_COLOR)
                .lens(Manga::title),
        )
        .fix_width(112.5)
        .background(Color::BLACK)
        .on_click(|ctx, data, _| {
            ctx.submit_command(cmd::NAVIGATE.with(Nav::MangaPage(data.url.clone())))
        })
}

pub fn mangas_widget() -> impl Widget<Vector<Manga>> {
    Scroll::new(GridView::new(manga::manga_widget).wrap().with_spacing(10.)).vertical()
}

pub fn manga_page_widget() -> impl Widget<Option<MangaDetail>> {
    Maybe::new(
        || {
            Flex::column().with_spacer(50.).with_flex_child(
                Flex::row()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_spacer(30.)
                    .with_child(
                        Flex::column()
                            .with_child(
                                RemoteImage::new(
                                    Spinner::new().fix_size(40., 40.).center(),
                                    |manga_detail: &MangaDetail, _| {
                                        Some(manga_detail.manga.icon_url.clone())
                                    },
                                )
                                .align_vertical(UnitPoint::TOP)
                                .fix_size(225., 325.)
                                .background(Color::BLACK),
                            )
                            .with_child(Button::new("Download").fix_width(225.))
                            .on_click(|ctx, data, _| {
                                for chapter in &data.chapters {
                                    ctx.submit_command(cmd::DOWNLOAD_CHAPTER.with(chapter.clone()))
                                }
                            }),
                    )
                    .with_spacer(30.)
                    .with_flex_child(
                        Flex::column()
                            .with_child(
                                Label::raw()
                                    .with_text_alignment(druid::TextAlignment::Start)
                                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                                    .with_text_size(theme::grid(5.))
                                    .with_text_color(theme::TEXT_COLOR)
                                    .lens(MangaDetail::manga.then(Manga::title)),
                            )
                            .with_flex_child(
                                FutureWidget::new(
                                    |data: &MangaDetail, _| {
                                        let manga = data.manga.clone();
                                        let connector = manga.connector.clone();
                                        async move {
                                            let mut chapters: Vector<Chapter> = Vector::new();
                                            let mut stream = GlobalAPI::global()
                                                .connectors
                                                .get(&connector)
                                                .unwrap()
                                                .get_chapters(manga);
                                            while let Some(res) = stream.next().await {
                                                chapters.push_front(res?)
                                            }
                                            Ok(chapters)
                                        }
                                    },
                                    Spinner::new().fix_size(50., 50.).center(),
                                    |value: Box<Result<Vector<Chapter>, Error>>,
                                     data: &mut MangaDetail,
                                     _| {
                                        if let Ok(chapters) = *value {
                                            data.chapters = chapters;
                                        }
                                        chapters_widget().boxed()
                                    },
                                ),
                                1.,
                            ),
                        FlexParams::new(1., CrossAxisAlignment::Start),
                    ),
                1.,
            )
        },
        || Label::new("Hello"),
    )
}
