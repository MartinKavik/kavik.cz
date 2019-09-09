use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;

pub fn view<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        view_content().els(),
        view_header().els(),
//        view_footer().els(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn view_header<Ms: 'static>() -> impl View<Ms> {
    vec![
        // Header background and line container
        div![
            class![
                C.fixed,
                C.top_0
                C.inset_x_0,
                C.h_16,
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
                    C.bg_gray_2,
                ]
            ],
        ],
        // Photo 1
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
                ],
                attrs!{
                    At::Src => "/static/images/photo_1.png"
                }
            ],
        ],
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
                    C.h_16
                    C.flex,
                    C.justify_between,
                    C.items_center,
                ],
                // Logo
                a![
                    attrs!{
                        At::Href => "/"
                    },
                    img![
                        class![
                            C.h_6
                        ],
                        attrs!{
                            At::Src => "/static/images/logo.svg"
                        }
                    ],
                ],
                // Hamburger
                div![
                    class![
                        C.cursor_pointer,
                    ],
                    img![
                        class![
                            C.h_8
                        ],
                        attrs!{
                            At::Src => "/static/images/hamburger.svg"
                        }
                    ]
                ]
            ],
            // Top header line
            div![
                class![
                    C.absolute,
                    C.top_0,
                    C.ml_12,
                    C.w_3of5,
                    C.h_px,
                    C.bg_gray_2,
                ]
            ],
        ],
    ]
}

pub fn view_content<Ms: 'static>() -> impl View<Ms> {
    vec![
        // Photo section
        section![
            class![
                C.w_screen,
                C.h_690px,
                C.bg_blue_10,
            ],
            // Small photo background container
            div![
                class![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.flex,
                    C.justify_center,
                ],
                // Small photo background
                div![
                    class![
                        C.w_xs,
                        C.h_300px,
                        C.bg_gray_1,
                    ]
                ],
            ],
            // Large photo background
            div![
                class![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.h_320px,
                    C.rounded_bl_140px,
                    C.bg_gray_1,
                ],
            ],
            // Gear
            img![
                class![
                    C.absolute
                    C.left_full,
                    C._ml_40,
                    C._mt_56,
                    C.w_md,
                    C.blur,
                ],
                attrs!{
                    At::Src => "/static/images/gear.svg"
                }
            ],
        ],
        // Developer section
        section![
            class![
                C.relative,
                C._mt_260px,
                C.h_1480px,
                C.pt_px,
                C.rounded_tr_140px,
                C.bg_gray_1,
                C.overflow_x_hidden,
            ],
            // Right background
            div![
                class![
                    C.absolute,
                    C.right_0,
                    C.inset_y_0,
                    C.w_76,
                    C.bg_yellow_4
                ]
            ],
            // I, developer
            h2![
                class![
                    C.relative,
                    C.mt_32,
                    C.mb_16,
                    C.text_center,
                    C.font_monospace,
                    C.font_bold,
                    C.text_40,
                    C.text_blue_10
                ],
                "I, developer"
            ],
            ul![
                class![
                    C.relative,
                    C.text_gray_8
                ],
                li![
                    class![
                        C.w_76,
                        C.pl_2,
                        C.pr_5,
                        C.py_10,
                        C.bg_gray_1,
                        C.flex,
                        C.flex_no_wrap,
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "I was working as a ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "backend"
                        ],
                        " developer in a bank and for some startups and agencies last years."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end
                    ],
                    div![
                        class![
                            C.w_76,
                            C.pl_2,
                            C.pr_5,
                            C.py_10,
                            C.flex,
                            C.flex_no_wrap,
                        ],
                        div![
                            class![
                                C.text_blue_6,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I'm also coming back to ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "frontend"
                            ],
                            " because it's finally possible to write reliable web apps and I want to make more use of my ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "artistic"
                            ],
                            " self."
                        ]
                    ]
                ],
                li![
                    class![
                        C.w_76,
                        C.pl_2,
                        C.pr_4,
                        C.py_10,
                        C.flex,
                        C.flex_no_wrap,
                        C.bg_gray_1
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "People make mistakes. That's why I setup linters, formatters, CI pipelines, etc., as ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "strict"
                        ],
                        " as possible and I want to write in ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Rust"
                        ],
                        ". Ideal tools \"bully\" me and don't have any configuration options."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end
                    ],
                    div![
                        class![
                            C.w_76,
                            C.pl_2,
                            C.pr_5,
                            C.py_10,
                            C.flex,
                            C.flex_no_wrap,
                        ],
                        div![
                            class![
                                C.text_blue_6,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I often learn from ",
                            span![
                                "packtpub.com"
                            ],
                            ". And I recommend to read book ",
                            span![
                                "Domain Modeling Made Functional"
                            ],
                            "."
                        ]
                    ]
                ],
            ]
        ],
        // Designer section
        section![
            class![
                C.relative,
                C._mt_260px,
                C.h_1000px,
                C.pt_px,
                C.rounded_tr_140px,
                C.bg_blue_10,
                C.overflow_hidden,
            ],
            // Circles
            div![
                class![
                    C.absolute,
                    C.left_1of2,
                    C._mt_12,
                    C._ml_545px,
                    C.w_1090px,
                    C.h_1090px,
                    C.opacity_10,
                ],
                div![
                    class![
                        C.absolute,
                        C.left_0,
                        C.bottom_0,
                        C.w_570px,
                        C.h_570px,
                        C.rounded_full,
                        C.border_yellow_4,
                        C.border_2
                    ]
                ],
                div![
                    class![
                        C.absolute,
                        C.right_0,
                        C.top_0,
                        C.w_570px,
                        C.h_570px,
                        C.rounded_full,
                        C.border_yellow_4,
                        C.border_2
                    ]
                ],
            ],
            div![
                class![
                    C.relative,
                    C.mt_32,
                    C.h_24,
                    C.bg_blue_10,
                    C.flex,
                    C.items_center,
                    C.justify_center,
                ],
                h2![
                    class![
                        C.text_center,
                        C.font_display,
                        C.font_semibold,
                        C.text_40,
                        C.text_yellow_4
                    ],
                    "I, designer"
                ],
            ],
            ul![
                class![
                    C.relative,
                    C.mt_16,
                    C.max_w_md,
                    C.mx_auto,
                    C.pl_1,
                    C.text_blue_1,
                    C.flex,
                    C.flex_col,
                ],
                li![
                    class![
                        C.w_76,
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_yellow_4,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "I've designed logos, my resume and this website in ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Affinity Designer"
                        ],
                        "."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end,
                    ],
                    div![
                        class![
                            C.mt_16,
                            C.ml_5,
                            C.w_64,
                            C.flex,
                            C.flex_no_wrap,
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I'll use ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "Figma"
                            ],
                            " for my next website design."
                        ]
                    ]
                ],
                li![
                    class![
                        C.mt_16,
                        C.w_76,
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_yellow_4,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "I have some experience with ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Adobe XD"
                        ],
                        ", ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Krita"
                        ],
                        " and ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Rhino3D"
                        ],
                        "."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end,
                    ],
                    div![
                        class![
                            C.mt_16,
                            C.ml_6,
                            C.w_76,
                            C.flex,
                            C.flex_no_wrap
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I recommend to check ",
                            span![
                                "refactoringui.com"
                            ],
                            ". I've bought their book and I use their ",
                            span![
                                "TailwindCSS"
                            ],
                            " in my projects."
                        ]
                    ]
                ],
            ]
        ],
        // Human section
        section![
            class![
                C.bg_blue_6
            ],
            h2![
                class![
                    C.font_ordinary,
                    C.font_bold,
                    C.text_40,
                    C.text_blue_2
                ],
                "I, human"
            ],
            div![
                class![
                    C.bg_blue_10
                ],
                h3![
                    class![
                        C.font_display,
                        C.font_thin,
                        C.text_35,
                        C.text_blue_3
                    ],
                    "Personal life"
                ],
                ul![
                    class![
                        C.text_blue_1
                    ],
                    li![
                        class![
                            C.flex,
                            C.flex_no_wrap
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I'm INTJ. When I'm not ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "creating"
                            ],
                            " something, I usually read or go to gym."
                        ]
                    ],
                    li![
                        class![
                            C.flex,
                            C.flex_no_wrap
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I like to spend my vacation at the cottage - hiking, cycling, driving, etc."
                        ]
                    ]
                ]
            ],
            div![
                class![
                    C.bg_blue_10
                ],
                h3![
                    class![
                        C.font_display,
                        C.font_thin,
                        C.text_35,
                        C.text_blue_3
                    ],
                    "Work life"
                ],
                ul![
                    class![
                        C.text_blue_1
                    ],
                    li![
                        class![
                            C.flex,
                            C.flex_no_wrap
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I'm ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "more productive"
                            ],
                            " when I'm working ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "remotely"
                            ],
                            "."
                        ]
                    ],
                    li![
                        class![
                            C.flex,
                            C.flex_no_wrap
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I like to ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "help"
                            ],
                            " people (not only on GitHub) and to mentor juniors."
                        ]
                    ],
                    li![
                        class![
                            C.flex,
                            C.flex_no_wrap
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                            ],
                            "▶"
                        ],
                        div![
                            "I'd rather think about your project for free in a gym than sit and wait for ideas. I also recommend to read ",
                            span![
                                "You don’t need standup"
                            ],
                            "."
                        ]
                    ]
                ]
            ]
        ],
        // Did you know section
        section![
            h2![
                class![
                    C.font_display,
                    C.font_semibold,
                    C.text_40,
                    C.text_gray_5
                ],
                "Did you know"
            ],
            ul![
                class![
                    C.text_gray_8
                ],
                li![
                    class![
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "I programmed a real football cannon."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "I jumped off a planeand a bridge."
                    ]
                ],
            ]
        ],
        // Want to meet section
        section![
            class![
                C.bg_blue_10
            ],
            ul![
                class![
                    C.text_blue_1
                ],
                li![
                    class![
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_yellow_4,
                            C.mr_1,
                        ],
                        "▶"
                    ],
                    div![
                        "Want to meet somewhere in ",
                        span![
                            class![
                                C.font_bold
                            ],
                            "Prague"
                        ],
                        "? Is there good coffee, tea, sushi or some spicy food? Ok! ",
                        span![
                            "martin@kavik.cz"
                        ]
                    ]
                ],
            ]
        ],
        // Resume section
        section![
            div![
                class![
                    C.text_19,
                    C.text_gray_10
                ],
                "Download my ",
                span![
                    class![
                        C.font_semibold
                    ],
                    "Resume"
                ],
                span![
                    class![
                        C.font_semibold,
                        C.text_gray_5
                    ],
                    ".pdf"
                ],
                div![
                    class![
                        C.text_yellow_6
                    ],
                    "V"
                ]
            ]
        ],
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
