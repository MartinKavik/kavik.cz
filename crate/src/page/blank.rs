use crate::{
    generated::css_classes::C,
    app::Msg,
};
use seed::prelude::*;
use seed::*;
use super::{view_header, view_footer, Page};

pub fn view() -> Vec<Node<Msg>> {
    vec![
        view_content().els(),
        view_header(Page::Other).els(),
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
