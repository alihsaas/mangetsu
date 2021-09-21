use druid::{
    widget::{Flex, Label, List, Scroll},
    UnitPoint, Widget, WidgetExt,
};

use crate::{core::Chapter, data::AppState, widgets::MyWidgetExt};

use super::theme;

pub fn chapter_widget() -> impl Widget<Chapter> {
    Flex::row().with_child(
        Label::raw()
            .with_text_alignment(druid::TextAlignment::Start)
            .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
            .with_text_color(theme::TEXT_COLOR)
            .lens(Chapter::title),
    )
}

pub fn chapters_widget() -> impl Widget<AppState> {
    Scroll::new(
        List::new(chapter_widget)
            .with_spacing(theme::grid(2.))
            .padding_right(theme::grid(1.))
            .lens(AppState::chapters),
    )
    .align_vertical(UnitPoint::TOP)
    .align_left()
    .expand_width()
}
