use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;

pub fn view<Ms: 'static>() -> impl View<Ms> {
    vec![
        h1![
           "PAGE NOT FOUND!"
        ],
        div![
            "loading..."
        ]
    ]
}