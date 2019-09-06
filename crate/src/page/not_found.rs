use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;

pub fn view<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        view_header().els(),
        view_content().els(),
        view_footer().els(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn view_header<Ms: 'static>() -> impl View<Ms> {
    header![class![C.text_27, C.text_gray_10], "MK"]
}

pub fn view_content<Ms: 'static>() -> impl View<Ms> {
    vec![
        h1![
            class![C.font_display, C.font_thin, C.text_29, C.text_gray_10],
            "PAGE NOT FOUND!"
        ],
        div!["loading..."],
    ]
}

pub fn view_footer<Ms: 'static>() -> impl View<Ms> {
    footer![div![
        div![
            span![class![C.text_gray_10], "MK"],
            span![
                class![C.font_display, C.font_semibold, C.text_15, C.text_yellow_6],
                "2019"
            ]
        ],
        div![
            class![C.font_display, C.font_semibold, C.text_16, C.text_gray_10],
            "martin@kavik.cz"
        ],
        div![class![C.text_yellow_6], "^"]
    ]]
}
