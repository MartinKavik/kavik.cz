use crate::{
    generated::css_classes::C,
    page,
    route::Route
};
use seed::{*, prelude::*, events::Listener};
use std::convert::TryInto;

// ------ ------
//     Model
// ------ ------

pub enum Page {
    Redirect,
    NotFound,
    Home,
    About,
}


impl Default for Page {
    fn default() -> Self {
        Page::Redirect
    }
}

#[derive(Default)]
pub struct ScrollPosition {
    pub current: i32,
    pub previous: i32
}

#[derive(Default)]
pub struct Model {
    pub page: Page,
    pub scroll_position: ScrollPosition,
    pub menu_visible: bool,
}

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Seed can't hydrate prerendered html (https://web.dev/prerender-with-react-snap)
    let mount_point = document()
        .get_element_by_id("app");

    if let Some(mount_point_element) = mount_point {
        mount_point_element.set_inner_html("");
    }

    orders
        .send_msg(Msg::RouteChanged(url.try_into().ok()));

    Model::default()
}

// ------ ------
// Window Events
// ------ ------

pub fn window_events(model: &Model) -> Vec<Listener<Msg>> {
    vec![
        raw_ev(Ev::Scroll, |_| {
            let mut position = seed::body().scroll_top();
            if position == 0 {
                position = seed::document()
                    .document_element()
                    .expect("cannot get document element")
                    .scroll_top()
            }
            Msg::Scrolled(position)
        })
    ]
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    RouteChanged(Option<Route>),
    ScrollToTop,
    Scrolled(i32),
    ToggleMenu,
    HideMenu,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::ScrollToTop => scroll_to_top(),
        Msg::Scrolled(position) => {
            model.scroll_position.previous = model.scroll_position.current;
            model.scroll_position.current = position;
        },
        Msg::ToggleMenu => {
            model.menu_visible = !model.menu_visible;
        },
        Msg::HideMenu => {
            model.menu_visible = false;
        },
    }
}

fn change_model_by_route(
    route: Option<Route>,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
) {
    match route {
        None => model.page = Page::NotFound,
        Some(route) => match route {
            Route::Home => {
                model.page = Page::Home;
            },
            Route::About => {
                model.page = Page::About;
            },
            Route::Redirect => {
                model.page = Page::Redirect;
            }
        },
    };
}

fn scroll_to_top() {
    seed::window().scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .top(0.)
            .left(0.)
            .behavior(web_sys::ScrollBehavior::Auto),
    )
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class![
//            C.fade_in,
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Redirect => page::blank::view(model),
            Page::Home => page::home::view(model),
            Page::About =>  page::about::view(model),
            Page::NotFound => page::not_found::view(model)
        }
    ]
}
