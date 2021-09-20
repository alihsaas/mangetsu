use std::sync::Arc;

use druid::{
    im::Vector,
    widget::{
        Button, CrossAxisAlignment, Either, Flex, Label, Painter, SizedBox, Spinner, Split,
        TextBox, ViewSwitcher,
    },
    Application, Color, Insets, Rect, RenderContext, Size, Target, Widget, WidgetExt, WindowState,
};
use futures::StreamExt;
use reqwest::Url;

use crate::core::{error::Error, Connectors, GlobalAPI, Manga};
use crate::data::{cmd, AppState, Nav};
use crate::theme;
use crate::widgets::{
    icons::{MAXIMIZED, QUIT_APP, RESTORED},
    FutureWidget, MyWidgetExt, ThemeScope, TitleBar,
};
use crate::{controller::NavController, data::Theme};

use super::manga::{manga_page_widget, mangas_widget};

fn titlebar() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(
            TitleBar::new(
                Label::dynamic(crate::ui::compute_window_title)
                    .with_text_size(12.5)
                    .with_text_color(theme::ICON_COLOR)
                    .center(),
            ),
            1.0,
        )
        .with_child(
            title_bar_button(
                Painter::new(|ctx, _, env| {
                    let size = ctx.size().to_vec2();
                    ctx.fill(
                        Rect::new(
                            size.x / 2. - 5.,
                            size.y / 2. - 0.5,
                            size.x / 2. + 5.,
                            size.y / 2. + 0.5,
                        ),
                        &env.get(theme::ICON_COLOR),
                    )
                })
                .center(),
            )
            .on_click(|ctx, _, _| {
                ctx.window()
                    .clone()
                    .set_window_state(WindowState::MINIMIZED)
            })
            .hover(theme::BACKGROUND_LIGHT),
        )
        .with_child(
            title_bar_button(Either::new(
                |data: &AppState, _| match *data.window_state {
                    WindowState::MAXIMIZED => true,
                    WindowState::RESTORED => false,
                    _ => false,
                },
                RESTORED.scale(Size::new(10., 10.)).center(),
                MAXIMIZED.scale(Size::new(10., 10.)).center(),
            ))
            .on_click(|ctx, data, _| {
                let mut window = ctx.window().clone();
                let new_state = match window.get_window_state() {
                    WindowState::MAXIMIZED => WindowState::RESTORED,
                    WindowState::RESTORED => WindowState::MAXIMIZED,
                    _ => WindowState::MAXIMIZED,
                };
                data.window_state = Arc::new(new_state);
                window.set_window_state(new_state);
            })
            .hover(theme::BACKGROUND_LIGHT),
        )
        .with_child(
            title_bar_button(
                QUIT_APP
                    .scale(Size::new(10., 10.))
                    .with_color(theme::ICON_COLOR)
                    .center(),
            )
            .on_click(|_, _, _| Application::global().quit())
            .hover(Color::rgb8(228, 16, 34)),
        )
        .background(theme::BACKGROUND_DARK)
}

fn title_bar_button(element: impl Widget<AppState> + 'static) -> impl Widget<AppState> {
    SizedBox::new(element).fix_width(46.).fix_height(20.)
}

pub fn app_widget() -> impl Widget<AppState> {
    let mut root = Flex::column();

    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(sidebar_menu_widget())
        //.with_default_spacer()
        .with_flex_spacer(1.)
        .with_child(
            Button::dynamic(|data: &AppState, _| {
                match data.theme {
                    Theme::Dark => "Light",
                    Theme::Light => "Dark",
                }
                .to_string()
            })
            .on_click(|_, data, _| {
                data.theme = match data.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                }
            })
            .align_right(),
        )
        .padding(if cfg!(target_os = "macos") {
            // Accommodate the window controls on Mac.
            Insets::new(0.0, 24.0, 0.0, 0.0)
        } else {
            Insets::ZERO
        })
        .background(theme::BACKGROUND_DARK);

    let main = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("Manga URL")
                        //.controller()
                        .lens(AppState::manga_search_url)
                        .fix_width(theme::grid(50.)),
                )
                .with_child(
                    Button::new("Search").on_click(|ctx, data: &mut AppState, _| {
                        let search_url = data.manga_search_url.clone();
                        let handle = ctx.get_external_handle();
                        tokio::spawn(async move {
                            let connector = dbg!(GlobalAPI::global().connectors.iter().find(
                                |(_, connector)| {
                                    connector.can_handle_uri(Url::parse(&search_url).unwrap())
                                }
                            ));
                            if let Some((_, connector)) = connector {
                                let manga = connector
                                    .get_manga_from_url(Url::parse(&search_url).unwrap())
                                    .await
                                    .unwrap();
                                handle
                                    .submit_command(
                                        cmd::NAVIGATE,
                                        Nav::MangaPage(manga),
                                        Target::Auto,
                                    )
                                    .unwrap();
                            }
                        });
                    }),
                ),
        )
        .with_flex_child(route_widget(), 1.0)
        .background(theme::BACKGROUND_LIGHT);

    let split = Split::columns(sidebar, main)
        .split_point(0.2)
        .bar_size(1.0)
        .min_size(150.0, 300.0)
        .min_bar_area(1.0)
        .solid_bar(true);

    root.add_child(titlebar());

    ThemeScope::new(root.with_flex_child(split, 1.)).controller(NavController)
    // .debug_invalidation()
    // .debug_widget_id()
    // .debug_paint_layout()
}

fn sidebar_menu_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_default_spacer()
        .with_child(sidebar_link_widget("Home", Nav::Home))
        .with_child(sidebar_link_widget("Downloads", Nav::Downloads))
}

fn sidebar_link_widget(title: &str, nav: Nav) -> impl Widget<AppState> {
    Label::new(title)
        .with_font(theme::UI_FONT_MEDIUM)
        .padding((theme::grid(2.0), theme::grid(1.0)))
        .expand_width()
        .link()
        .env_scope({
            let nav = nav.clone();
            move |env, route: &Nav| {
                env.set(
                    theme::LINK_COLD_COLOR,
                    if &nav == route {
                        env.get(theme::MENU_BUTTON_BG_ACTIVE)
                    } else {
                        env.get(theme::MENU_BUTTON_BG_INACTIVE)
                    },
                );
                env.set(
                    theme::TEXT_COLOR,
                    if &nav == route {
                        env.get(theme::MENU_BUTTON_FG_ACTIVE)
                    } else {
                        env.get(theme::MENU_BUTTON_FG_INACTIVE)
                    },
                );
            }
        })
        .on_click(move |ctx, _, _| {
            ctx.submit_command(cmd::NAVIGATE.with(nav.clone()));
        })
        .lens(AppState::route)
}

fn route_widget() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _| data.route.clone(),
        |value: &Nav, _, _| match value {
            Nav::Home => home_widget().boxed(),
            Nav::Downloads => Label::new("No").boxed(),
            Nav::MangaPage(manga) => manga_page_widget(manga).boxed(),
        },
    )
}

//TODO: Make this widget stream the result instead of collecting them.
fn home_widget() -> impl Widget<AppState> {
    FutureWidget::new(
        |_, _| async {
            let mut vectors: Vector<Manga> = Vector::new();
            let mut stream = GlobalAPI::global()
                .connectors
                .get(&Connectors::Manganel)
                .unwrap()
                .get_mangas_from_page(1);
            while let Some(res) = stream.next().await {
                vectors.push_front(res?)
            }
            Ok(vectors)
        },
        Spinner::new().fix_size(50., 50.).center(),
        |value: Box<Result<Vector<Manga>, Error>>, data: &mut AppState, _| {
            if let Ok(mangas) = *value {
                //panic!("{:?}", value);
                data.mangas = mangas;
            }
            mangas_widget().lens(AppState::mangas).boxed()
        },
    )
}
