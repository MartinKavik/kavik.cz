use crate::{
    generated::css_classes::C,
    page,
    route::Route
};
use seed::prelude::*;
use seed::*;
use std::convert::TryInto;

// ------ ------
//     Model
// ------ ------

pub enum Model {
    Redirect,
    NotFound,
    Home,
    About,
}

impl Default for Model {
    fn default() -> Self {
        Model::Redirect
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mount_point = document()
        .get_element_by_id("app");

    if let Some(mount_point_element) = mount_point {
        mount_point_element.set_inner_html("");
    }

    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));
    Model::default()
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    RouteChanged(Option<Route>),
    ScrollToTop,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::ScrollToTop => scroll_to_top()
    }
}

fn change_model_by_route(
    route: Option<Route>,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
) {
    match route {
        None => *model = Model::NotFound,
        Some(route) => match route {
            Route::Home => {
                *model = Model::Home;
            },
            Route::About => {
                *model = Model::About;
            },
            Route::Redirect => {
                *model = Model::Redirect;
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
        match model {
            Model::Redirect => page::blank::view(),
            Model::Home => page::home::view(),
            Model::About =>  page::about::view(),
            Model::NotFound => page::not_found::view()
        }
    ]
}
