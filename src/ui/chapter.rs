use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    UnitPoint, Widget, WidgetExt,
};

use crate::{
    core::Chapter,
    data::{cmd::DOWNLOAD_CHAPTER, MangaDetail},
    widgets::{DynamicSizedBox, MyWidgetExt},
};

use super::theme;

pub fn chapter_widget() -> impl Widget<Chapter> {
    Flex::row()
        .with_child(
            Button::new("Download").on_click(|ctx, data: &mut Chapter, _| {
                ctx.submit_command(DOWNLOAD_CHAPTER.with(data.clone()))
            }),
        )
        .with_flex_child(
            DynamicSizedBox::new(
                Label::raw()
                    .with_text_alignment(druid::TextAlignment::Start)
                    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
                    .with_text_color(theme::TEXT_COLOR)
                    .lens(Chapter::title),
            ),
            1.,
        )
}

pub fn chapters_widget() -> impl Widget<MangaDetail> {
    Scroll::new(
        List::new(chapter_widget)
            .with_spacing(theme::grid(2.))
            .padding_right(theme::grid(1.))
            .expand_width()
            .lens(MangaDetail::chapters),
    )
    .vertical()
    .align_vertical(UnitPoint::TOP)
    .align_left()
    .expand_width()
}
