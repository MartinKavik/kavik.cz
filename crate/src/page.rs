use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;

pub mod about;
pub mod home;
pub mod not_found;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    Other
}

fn view_header<Ms: 'static>(page: Page) -> impl View<Ms> {
    vec![
        // Header background and line container
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
        ],
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
                ],
                img![
                    class![
                        C.w_xs,
                        C.h_full,
                        C.object_contain,
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
                        C.sm__flex,
                        C.sm__text_21,
                        C.sm__font_semibold,
                        C.sm__text_gray_10
                    ],
                    li![
                        class![
                        ],
                        "Home & Projects"
                    ],
                    li![
                        class![
                            // sm__
                            C.sm__ml_8
                        ],
                        "About"
                    ]
                ],
                // Hamburger
                div![
                    class![
                        C.cursor_pointer,
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
                ]
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
        ],
    ]
}

pub fn view_footer<Ms: 'static>() -> impl View<Ms> {
    footer![
        class![
            C.h_16,
            C.shadow_2xl_above,
            C.flex,
            C.justify_center,
        ],
        div![
            class![
                C.w_xs,
                C.h_full,
                C.px_5,
                C.flex,
                C.justify_between,
                C.items_center,
            ],
            div![
                img![
                    class![
                        C.w_6,
                    ],
                    attrs!{
                        At::Src => "/static/images/logo.svg"
                    }
                ],
                span![
                    class![C.ml_1, C.font_display, C.font_semibold, C.text_15, C.text_yellow_6],
                    "2019"
                ]
            ],
            div![
                class![C.font_display, C.font_semibold, C.text_16, C.text_gray_10],
                "martin@kavik.cz"
            ],
            img![
                class![
                    C.mt_1,
                    C.w_12,
                ],
                attrs!{
                    At::Src => "/static/images/top.svg"
                }
            ],
        ]
    ]
}
