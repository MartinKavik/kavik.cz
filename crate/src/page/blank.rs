use crate::{
    generated::css_classes::C,
    app::{Model, Msg},
};
use seed::prelude::*;
use seed::*;
use super::{view_header, view_footer, Page};

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    seed::document().set_title("Kavik.cz");

    vec![
        view_content().els(),
        view_header(model, Page::Other).els(),
        view_footer().els(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn view_content() -> impl View<Msg> {
    div![
        class![
            C.flex_grow,
        ],
    ]
}
