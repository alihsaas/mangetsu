use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, Scroll, Spinner},
    Color, UnitPoint, Widget, WidgetExt,
};

use crate::{
    core::Manga,
    data::{cmd, AppState, Nav},
    widgets::{remote_image::RemoteImage, DynamicSizedBox, GridView},
};

use super::{manga, theme};

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
                .with_text_color(theme::TEXT_COLOR)
                .lens(Manga::title),
        )
        .fix_width(112.5)
        .background(Color::BLACK)
        .on_click(|ctx, data, _| {
            ctx.submit_command(cmd::NAVIGATE.with(Nav::MangaPage(data.clone())))
        })
}

pub fn mangas_widget() -> impl Widget<Vector<Manga>> {
    Scroll::new(GridView::new(manga::manga_widget).wrap().with_spacing(10.)).vertical()
}

pub fn manga_page_widget(manga: &Manga) -> impl Widget<AppState> {
    let manga = manga.clone();
    Flex::column().with_spacer(50.).with_flex_child(
        Flex::row()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_spacer(30.)
            .with_child(
                Flex::column()
                    .with_child(
                        RemoteImage::new(
                            Spinner::new().fix_size(40., 40.).center(),
                            move |_, _| Some(manga.icon_url.to_string().into()),
                        )
                        .align_vertical(UnitPoint::TOP)
                        .fix_size(225., 325.)
                        .background(Color::BLACK),
                    )
                    .with_child(Button::new("Download").fix_width(225.)),
            )
            .with_spacer(30.)
            .with_flex_child(
                Flex::column().with_flex_child(
                    DynamicSizedBox::new(
                        Label::new(manga.title)
                            .with_text_alignment(druid::TextAlignment::Start)
                            .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                            .with_text_size(theme::grid(5.))
                            .with_text_color(theme::TEXT_COLOR),
                    ),
                    1.,
                ),
                1.,
            ),
        1.,
    )
}
