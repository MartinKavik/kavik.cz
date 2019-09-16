use crate::{
    generated::css_classes::C,
    app::{Model, Msg},
};
use seed::prelude::*;
use seed::*;

pub mod about;
pub mod home;
pub mod not_found;
pub mod blank;

const MAILTO: &str = "mailto:martin@kavik.cz?subject=Something%20for%20Martin&body=Hi!%0A%0AI%20am%20Groot.%20I%20like%20trains.";

#[derive(Clone, Copy, Eq, PartialEq)]
enum Page {
    Home,
    About,
    Other
}

fn view_header(model: &Model, page: Page) -> impl View<Msg> {
    let show_header =
        model.scroll_position.current <= 0 ||
        model.scroll_position.current <= model.scroll_position.previous;

    vec![
        // Header background and line container
        if !show_header {
            empty![]
        } else {
            div![
                class![
                    C.fixed,
                    C.top_0
                    C.inset_x_0,
                    C.h_16,
                    // sm__
                    C.sm__h_24,
                ],
                // Header background
                div![
                    class![
                        C.absolute,
                        C.inset_0,
                        C.bg_gray_1,
                        C.opacity_90
                    ]
                ],
                // Bottom header line
                div![
                    class![
                        C.absolute,
                        C.bottom_0,
                        C.ml_12,
                        C.w_3of5,
                        C.h_px,
                        // sm__
                        C.sm__ml_0,
                        C.sm__w_full,
                        C.sm__h_3px
                        C.sm__flex,
                        C.sm__justify_center,
                        // md__
                        C.md___ml_30,
                    ],
                    div![
                        class![
                            C.h_full,
                            C.bg_gray_2,
                            // sm__
                            C.sm__ml_6,
                            C.sm__w_48,
                            C.sm__bg_yellow_6 => page == Page::Home,
                        ],
                    ],
                    div![
                        class![
                            C.hidden,
                            // sm__
                            C.sm__block,
                            C.sm__h_full,
                            C.sm__bg_gray_2,
                            C.sm__w_24,
                            C.sm__bg_yellow_6 => page == Page::About,
                        ],
                    ]
                ],
            ]
        },
        // Photo 1
        if let Page::About = page {
            div![
                class![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.mt_6,
                    C.flex,
                    C.justify_center
                    // sm__
                    C.sm__mt_10,
                    // md__
                    C.md__mt_8,
                ],
                img![
                    class![
                        C.w_xs,
                        C.h_full,
                        C.object_contain,
                        // sm__
                        C.sm__w_100,
                        // lg
                        C.lg__w_570px,
                    ],
                    attrs!{
                        At::Src => "/static/images/photo_1.png"
                    }
                ],
            ]
        } else {
            empty![]
        },
        // Header
        if !show_header {
            empty![]
        } else {
            header![
                class![
                    C.fixed,
                    C.top_0
                    C.inset_x_0,
                ],
                // Header controls container
                div![
                    class![
                        C.mx_8
                        C.h_16,
                        C.flex,
                        C.justify_between,
                        C.items_center,
                        // sm__
                        C.sm__h_24,
                    ],
                    // Logo
                    a![
                        attrs!{
                            At::Href => "/"
                        },
                        simple_ev(Ev::Click, Msg::ScrollToTop),
                        img![
                            class![
                                C.h_6,
                                // sm__
                                C.sm__h_10,
                            ],
                            attrs!{
                                At::Src => "/static/images/logo.svg"
                            }
                        ],
                    ],
                    // Links
                    ul![
                        class![
                            C.hidden,
                            // sm__
                            C.sm___mt_px,
                            C.sm__text_21,
                            C.sm__font_semibold,
                            C.sm__text_gray_10,
                            C.sm__flex,
                            C.sm__items_center,
                            C.sm__h_full,
                        ],
                        li![
                            class![
                                // sm__
                                C.sm__block,
                                C.sm__h_full,
                            ],
                            a![
                                class![
                                    // sm__
                                    C.sm__h_full,
                                    C.sm__flex,
                                    C.sm__items_center,
                                    C.sm__hover__text_yellow_7,
                                    C.sm__outline_none,
                                ],
                                attrs!{
                                    At::Href => "/"
                                },
                                simple_ev(Ev::Click, Msg::ScrollToTop),
                                "Home & Projects"
                            ]
                        ],
                        li![
                            class![
                                // sm__
                                C.sm__block,
                                C.sm__ml_8
                                C.sm__h_full,
                            ],
                            a![
                                class![
                                    // sm__
                                    C.sm__h_full,
                                    C.sm__flex,
                                    C.sm__items_center,
                                    C.sm__hover__text_yellow_7,
                                    C.sm__outline_none,
                                ],
                                attrs!{
                                    At::Href => "/about"
                                },
                                simple_ev(Ev::Click, Msg::ScrollToTop),
                                "About"
                            ]
                        ],
                        li![
                            class![
                                C.hidden,
                                // md__
                                C.md__block,
                                C.md__ml_12,
                                C.md__h_full,
                            ],
                            a![
                                class![
                                    // md__
                                    C.md__h_full,
                                    C.md__flex,
                                    C.md__items_center,
                                    C.md__hover__text_yellow_7,
                                ],
                                attrs!{
                                    At::Href => "/static/Martin_Kavik_resume.pdf"
                                },
                                "Resume",
                                span![
                                    class![
                                        // md__
                                        C.md__text_gray_5,
                                    ],
                                    ".pdf"
                                ]
                            ]
                        ],
                        li![
                            class![
                                C.hidden,
                                // md__
                                C.md__block,
                                C.md__ml_8,
                                C.md__h_full,
                            ],
                            a![
                                class![
                                    // md__
                                    C.md__h_full,
                                    C.md__flex,
                                    C.md__items_center,
                                    C.md__hover__text_yellow_7,
                                ],
                                attrs!{
                                    At::Href => "https://github.com/MartinKavik"
                                },
                                "GitHub",
                                img![
                                    class![
                                        // md__
                                        C.md__inline
                                        C.md__mb_5,
                                        C.md__ml_px,
                                        C.md__w_4,
                                    ],
                                    attrs!{
                                        At::Src => "/static/images/link_arrow.svg"
                                    }
                                ]
                            ]
                        ],
                    ],
                    // Hamburger
                    div![
                        class![
                            C.cursor_pointer,
                            // md__
                            C.md__hidden,
                        ],
                        img![
                            class![
                                C.h_8,
                                // sm__
                                C.sm__h_10
                            ],
                            attrs!{
                                At::Src => "/static/images/hamburger.svg"
                            }
                        ]
                    ],
                    // Spacer
                    div![
                        class![
                            C.hidden,
                            // md__
                            C.md__block,
                        ],
                    ],
                ],
                // Bottom header line
                div![
                    class![
                        C.absolute,
                        C.top_0,
                        C.ml_12,
                        C.w_3of5,
                        C.h_px,
                        // sm__
                        C.sm__ml_0,
                        C.sm__w_full,
                        C.sm__h_3px
                        C.sm__flex,
                        C.sm__justify_center,
                        // md__
                        C.md___ml_30,
                    ],
                    div![
                        class![
                            C.h_full,
                            C.bg_gray_2,
                            // sm__
                            C.sm__ml_6,
                            C.sm__w_48,
                            C.sm__bg_yellow_6 => page == Page::Home,
                        ],
                    ],
                    div![
                        class![
                            C.hidden,
                            // sm__
                            C.sm__block,
                            C.sm__h_full,
                            C.sm__bg_gray_2,
                            C.sm__w_24,
                            C.sm__bg_yellow_6 => page == Page::About,
                        ],
                    ]
                ],
            ]
        }
    ]
}

pub fn view_footer() -> impl View<Msg> {
    footer![
        class![
            C.h_16,
            C.shadow_2xl_above,
            C.flex,
            C.justify_center,
            // sm__
            C.sm__h_24,
        ],
        div![
            class![
                C.w_xs,
                C.h_full,
                C.px_5,
                C.flex,
                C.justify_between,
                C.items_center,
                // sm__
                C.sm__w_132
            ],
            div![
                class![
                    // lg__
                    C.lg__pb_3,
                ],
                img![
                    class![
                        C.inline,
                        C.w_6,
                        C.align_baseline,
                        // sm__
                        C.sm__w_12
                    ],
                    attrs!{
                        At::Src => "/static/images/logo.svg"
                    }
                ],
                span![
                    class![
                        C.ml_1,
                        C.font_display,
                        C.font_semibold,
                        C.text_15,
                        C.text_yellow_6,
                        // sm__
                        C.sm__mt_2,
                        C.sm__text_25,
                    ],
                    "2019"
                ]
            ],
            a![
                attrs!{
                    At::Href => MAILTO,
                },
                class![
                    C.font_display,
                    C.font_semibold,
                    C.text_16,
                    C.text_gray_10,
                    C.underline,
                    C.underline_yellow_7,
                    // sm__
                    C.sm__text_26
                ],
                "martin@kavik.cz"
            ],
            div![
                class![
                    C.cursor_pointer,
                    C.h_full,
                    C.flex,
                    C.items_center,
                ],
                simple_ev(Ev::Click, Msg::ScrollToTop),
                img![
                    class![
                        C.mt_1,
                        C.w_12,
                        // sm__
                        C.sm__w_16
                    ],
                    attrs!{
                        At::Src => "/static/images/top.svg"
                    }
                ],
            ]
        ]
    ]
}
