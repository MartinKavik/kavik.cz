use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;
use super::{view_header, view_footer, Page};

pub fn view<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        view_content().els(),
        view_header(Page::Home).els(),
        view_footer().els(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn view_content<Ms: 'static>() -> impl View<Ms> {
    vec![
        // Main section
        section![
            class![
                C.relative,
                C.h_690px,
                C.bg_gray_1,
                // sm__
                C.sm__h_890px,
            ],
            // Left background
            div![
                class![
                    C.absolute,
                    C.left_0,
                    C.inset_y_0,
                    C.w_1of2,
                    C.bg_yellow_4,
                ]
            ],
            div![
                class![
                    C.relative,
                    C.flex,
                    C.justify_center,
                ],
                // Martin Kavík container
                div![
                    class![
                        C.h_360px,
                        C.rounded_bl_90px,
                        C.bg_gray_1,
                        // sm__
                        C.sm__h_550px,
                    ],
                    // Martin Kavík
                    div![
                        class![
                            C.mt_40,
                            C.ml_12,
                            C.w_xs,
                            C.font_display,
                            // sm__
                            C.sm__mt_64,
                            C.sm__ml_16,
                            C.sm__w_md,
                        ],
                        h1![
                            class![
                                C.inline,
                                C.leading_tight,
                                C.text_31,
                                C.text_gray_10
                                // sm__
                                C.sm__text_40
                            ],
                            span![
                                "Martin "
                            ],
                            span![
                                class![
                                    C.font_bold
                                ],
                                "Kavík"
                            ],
                        ],
                        span![
                            class![
                                C.text_21,
                                C.text_gray_7,
                                // sm__
                                C.sm__text_32
                            ],
                            "\u{00A0}is",
                            br![],
                            "a developer",
                            br![],
                            "with 7+ years of experience",
                            br![],
                            "who likes to design and ..."
                        ]
                    ],
                ],
            ],
            // Gear
            img![
                class![
                    C.absolute
                    C.top_0,
                    C.left_full,
                    C._ml_40,
                    C._mt_56,
                    C.w_md,
                    C.max_w_none,
                    C.blur,
                    // sm__
                    C.sm___mt_64,
                    C.sm__w_750px,
                ],
                attrs!{
                    At::Src => "/static/images/gear.svg"
                }
            ],
        ],
        // Seed section
        section![
            class![
                C.relative,
                C._mt_48,
                C.pt_px,
                C.rounded_tl_140px,
                C.bg_blue_8,
            ],
            // Main list
            div![
                class![
                    C._ml_5,
                    C._mt_48,
                    C.max_w_lg,
                    C.flex,
                    C.justify_end,
                    C.relative,
                    // sm__
                    C.max_w_4xl,
                ],
                // Right background
                div![
                    class![
                        C.absolute,
                        C.right_0,
                        C.inset_y_0,
                        C._right_50vw,
                        C.w_50vw,
                        C.bg_gray_1
                    ]
                ],
                // List
                div![
                    class![
                        C.relative,
                        C.pl_4,
                        C.rounded_bl_45px,
                        C.font_display,
                        C.text_17,
                        C.text_gray_8,
                        C.bg_gray_1,
                        C.overflow_hidden,
                        // sm__
                        C.sm__pl_8,
                        C.sm__pr_2,
                        C.sm__text_26,
                    ],
                    ul![
                        class![
                            C.w_xs,
                            C.pl_2,
                            C.py_8,
                            // sm__
                            C.sm__w_120,
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_blue_6,
                                        C.mr_2,
                                    ],
                                    // https://stackoverflow.com/a/39900080
                                    "▶\u{fe0e}"
                                ],
                                "To work on your project."
                            ]
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_blue_6,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "Readable code and UI."
                            ]
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_blue_6,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "Rust, Affinity Designer and Figma."
                            ]
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_blue_6,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "Receiving mails. ",
                                    span![
                                        class![
                                            C.font_semibold
                                        ],
                                        "martin@kavik.cz"
                                    ]
                                ]
                            ]
                        ],
                    ]
                ]
            ],
            div![
                class![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    class![
                        C.mt_20,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_48,
                        C.sm__w_md,
                    ],
                    // Github projects
                    h2![
                        class![
                            C.font_display,
                            C.text_23,
                            C.text_blue_3,
                            C.text_center,
                            // sm__
                            C.sm__text_50,
                        ],
                        span![
                            class![
                                C.font_thin
                            ],
                            "TOP-5"
                        ],
                        span![
                            class![
                                C.font_normal
                            ],
                            " GITHUB PROJECTS"
                        ]
                    ],
                    // Testimonial 1
                    div![
                        class![
                            C.mt_20,
                            C.text_right,
                            // sm__
                            C.sm__mt_40,
                        ],
                        div![
                            class![
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Awesome, awesome framework!"
                        ],
                        div![
                            class![
                                C.mt_2,
                                C.mr_2,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__mt_4,
                                C.sm__text_20,
                            ],
                            "- rebo"
                        ]
                    ],
                    // Testimonial 2
                    div![
                        class![
                            C.mt_8,
                            C.text_right,
                            // sm__
                            C.sm__mt_12,
                        ],
                        div![
                            class![
                                C.mr_4,
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Seed rocks, and ",
                            br![],
                            "Martin makes it better."
                        ],
                        div![
                            class![
                                C.mt_2,
                                C.mr_5,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__mt_4,
                                C.sm__text_20,
                            ],
                            "- robwebbjr"
                        ]
                    ],
                    // Seed
                    div![
                        class![
                            C.relative,
                            C.mt_8,
                            C.pt_5,
                            C.pb_3,
                            C.w_36,
                            C.rounded_tr_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_24,
                            C.sm__pt_8,
                            C.sm__pb_8,
                            C.sm__w_64,
                            C.sm__rounded_tr_55px,
                        ],
                        // Extended background
                        div![
                            class![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        img![
                            class![
                                C.h_18,
                                C.max_w_none,
                                // sm__
                                C.sm__h_32
                            ],
                            attrs!{
                                At::Src => "/static/images/seed_logo.svg"
                            }
                        ]
                    ],
                    ul![
                        class![
                            C.mt_10,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_20,
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    h3![
                                        class![
                                            C.inline,
                                            C.text_18,
                                            C.font_bold,
                                            // sm__
                                            C.sm__text_26,
                                        ],
                                        "Seed"
                                    ],
                                    "\u{00A0}is an open-source Rust framework for creating fast and reliable web apps running in WebAssembly."
                                ]
                            ]
                        ],
                        li![
                            class![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "I'm the main contributor."
                            ]
                        ],
                        li![
                            class![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "I've designed the logo."
                            ]
                        ],
                    ],
                    div![
                        class![
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_16,
                            C.sm__mb_48,
                        ],
                        span![
                            class![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            class![
                                C.text_blue_2
                            ],
                            "awesome-seed-rs"
                        ],
                        img![
                            class![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                            ],
                            attrs!{
                                At::Src => "/static/images/link_arrow.svg"
                            }
                        ]
                    ]
                ]
            ]
        ],
        // RealWorld section
        section![
            class![
                C.bg_blue_10
            ],
            div![
                class![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    class![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_132,
                    ],
                    // Testimonial
                    div![
                        class![
                            C.mt_10,
                            C.mr_2,
                            C.text_right,
                        ],
                        div![
                            class![
                                C.mr_3,
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Your real world example really is the mother of all examples."
                        ],
                        div![
                            class![
                                C.mt_4,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__mt_6,
                                C.sm__text_20,
                            ],
                            "- theduke"
                        ]
                    ],
                    // RealWorld example app
                    div![
                        class![
                            C.relative,
                            C.mt_12,
                            C.ml_1,
                            C.pt_8,
                            C.pb_6,
                            C.rounded_tl_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_32,
                            C.sm__pt_12,
                            C.sm__pb_10,
                            C.sm__rounded_tl_55px,
                        ],
                        // Extended background
                        div![
                            class![
                                C.absolute,
                                C._right_50vw,
                                C.inset_y_0,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                       img![
                            class![
                                C.h_12,
                                C.max_w_none,
                                // sm__
                                C.sm__h_20
                            ],
                            attrs!{
                                At::Src => "/static/images/realworld_logo.png"
                            }
                        ]
                    ],
                    ul![
                        class![
                            C.mt_12,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_24,
                            C.sm__ml_8,
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    h3![
                                        class![
                                            C.inline,
                                            C.text_18,
                                            C.font_bold,
                                            // sm__
                                            C.sm__text_26,
                                        ],
                                        "RealWorld example"
                                    ],
                                    "\u{00A0}is a Seed codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the RealWorld spec and API."
                                ]
                            ]
                        ],
                    ],
                    div![
                        class![
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_20,
                            C.sm__mb_48,
                        ],
                        span![
                            class![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            class![
                                C.text_blue_2
                            ],
                            "seed-rs-realworld"
                        ],
                        img![
                            class![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                            ],
                            attrs!{
                                At::Src => "/static/images/link_arrow.svg"
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Kavik.cz section
        section![
            class![
                C.bg_blue_6
            ],
            div![
                class![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    class![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_md,
                    ],
                    // Testimonial
                    div![
                        class![
                            C.mt_10,
                            C.mr_3,
                            C.text_right
                        ],
                        div![
                            class![
                                C.mr_4,
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Fork it, use it!"
                        ],
                        div![
                            class![
                                C.mt_4,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__text_20,
                            ],
                            "- me"
                        ]
                    ],
                    // MK
                    div![
                        class![
                            C.relative,
                            C.mt_8,
                            C.pt_4,
                            C.pb_2,
                            C.w_36,
                            C.rounded_tr_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_20,
                            C.sm__w_56,
                            C.sm__pt_10,
                            C.sm__pb_8,
                            C.sm__rounded_tr_55px,
                        ],
                        // Extended background
                        div![
                            class![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        img![
                            class![
                                C.ml_6,
                                C.h_20,
                                C.max_w_none,
                                // sm__
                                C.sm__h_32,
                            ],
                            attrs!{
                                At::Src => "/static/images/logo.svg"
                            }
                        ]
                    ],
                    ul![
                        class![
                            C.mt_10,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_24,
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    h3![
                                        class![
                                            C.inline,
                                            C.text_18,
                                            C.font_bold,
                                            // sm__
                                            C.sm__text_26,
                                        ],
                                        "kavik.cz"
                                    ],
                                    "\u{00A0}is this website."
                                ]
                            ]
                        ],
                        li![
                            class![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "You can fork it, modify it and use it as your own website."
                            ]
                        ],
                    ],
                    div![
                        class![
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_20,
                            C.sm__mb_56,
                        ],
                        span![
                            class![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            class![
                                C.text_blue_2
                            ],
                            "kavik.cz"
                        ],
                        img![
                            class![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                            ],
                            attrs!{
                                At::Src => "/static/images/link_arrow.svg"
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Seed Quickstart section
        section![
            class![
                C.rounded_br_140px,
                C.bg_blue_10
            ],
            div![
                class![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    class![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_132,
                    ],
                    // Testimonial
                    div![
                        class![
                            C.mt_10,
                            C.ml_5,
                            // sm__
                            C.sm__ml_12,
                        ],
                        div![
                            class![
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Its great!"
                        ],
                        div![
                            class![
                                C.ml_12,
                                C.mt_4,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__ml_20,
                                C.sm__text_20,
                            ],
                            "- rebo"
                        ]
                    ],
                    // Seed Quickstart
                    div![
                        class![
                            C.relative,
                            C.mt_12,
                            C.ml_6,
                            C.pt_8,
                            C.pb_6,
                            C.rounded_tl_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                             // sm__
                            C.sm__mt_32,
                            C.sm__pt_12,
                            C.sm__pb_10,
                            C.sm__rounded_tl_55px,
                        ],
                        // Extended background
                        div![
                            class![
                                C.absolute,
                                C._right_50vw,
                                C.inset_y_0,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        div![
                            class![
                                C.ml_2,
                                C.font_display,
                                C.font_bold,
                                C.text_34,
                                 // sm__
                                C.sm__ml_4,
                                C.sm__text_60,
                            ],
                            "Seed Quickstart"
                        ],
                    ],
                    ul![
                        class![
                            C.mt_12,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_24,
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_yellow_4,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    h3![
                                        class![
                                            C.inline,
                                            C.text_18,
                                            C.font_bold,
                                            // sm__
                                            C.sm__text_26,
                                        ],
                                        "Seed Quickstart"
                                    ],
                                    "\u{00A0}is a template for web apps with Seed, TailwindCSS, Typescript and Webpack."
                                ]
                            ]
                        ],
                    ],
                    div![
                        class![
                            C.mt_10,
                            C.mb_56,
                            C.text_right,
                            C.font_display,
                            C.text_15,
                            // sm__
                            C.sm__mt_20,
                            C.sm__mb_96,
                            C.sm__text_26,
                        ],
                        span![
                            class![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            class![
                                C.text_blue_2
                            ],
                            "seed-quickstart-webpack"
                        ],
                        img![
                            class![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                            ],
                            attrs!{
                                At::Src => "/static/images/link_arrow.svg"
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Hellweb section
        section![
            div![
                class![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    class![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_132,
                    ],
                    // Hellweb container
                    div![
                        class![
                            C.relative,
                            C._mt_40,
                            C.w_xs,
                            C.py_12,
                            C.rounded_tr_140px,
                            C.bg_gray_1,
                            // sm__
                            C.sm__w_132,
                            C.sm___mt_80,
                        ],
                        // Extended background
                        div![
                            class![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_gray_1,
                            ]
                        ],
                        // Hellweb
                        img![
                            class![
                                C.mt_1,
                                C.ml_8,
                                C.h_12,
                                // sm__
                                C.sm__mt_20,
                                C.sm__h_20,
                            ],
                            attrs!{
                                At::Src => "/static/images/hellweb_logo.svg"
                            }
                        ],
                    ],
                    ul![
                        class![
                            C.text_gray_10,
                            // sm__
                            C.sm__mt_16,
                        ],
                        li![
                            class![
                                C.my_3,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_blue_6,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    h3![
                                        class![
                                            C.inline,
                                            C.text_18,
                                            C.font_bold,
                                            // sm__
                                            C.sm__text_26,
                                        ],
                                        "Hellweb"
                                    ],
                                    "\u{00A0}will be a collection of Rust libraries and applications which solve your pain points and explore new ideas."
                                ]
                            ]
                        ],
                        li![
                            class![
                                C.mt_8,
                                // sm__
                                C.sm__mt_16,
                            ],
                            div![
                                class![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    class![
                                        C.text_blue_6,
                                        C.mr_2,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "What do you hate or what annoys you about ",
                                    span![
                                        class![
                                            C.font_bold,
                                        ],
                                        "web design & development"
                                    ],
                                    "? Don't hesitate to create an issue or contact me - ",
                                    span![
                                        "martin@kavik.cz"
                                    ]
                                ]
                            ]
                        ]
                    ],
                    div![
                        class![
                            C.mt_10,
                            C.mr_2,
                            C.text_right,
                            // sm__
                            C.sm__mt_20,
                        ],
                        span![
                            class![
                                C.text_gray_5
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            class![
                                C.text_gray_9
                            ],
                            "hellweb-pain"
                        ],
                        img![
                            class![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                            ],
                            attrs!{
                                At::Src => "/static/images/link_arrow.svg"
                            }
                        ]
                    ],
                    // About your new developer
                    div![
                        class![
                            C.mt_20,
                            C.mb_16,
                            C.ml_5,
                            C.flex,
                            C.items_center,
                            C.justify_center,
                            C.text_19,
                            C.text_gray_10,
                            // sm__
                            C.sm__mt_40,
                            C.sm__mb_24,
                            C.sm__text_28,
                        ],
                        span![
                            class![
                                C.font_semibold
                            ],
                            "About"
                        ],
                        "\u{00A0}your new developer",
                        img![
                            class![
                                C.mt_1,
                                C.h_12,
                                // sm__
                                C.sm__h_16,
                            ],
                            attrs!{
                                At::Src => "/static/images/next.svg"
                            }
                        ],
                    ]
                ]
            ]
        ],
        // Circles
        div![
            class![
                C.absolute,
                C.left_1of2,
                C.top_0,
                C.mt_310px,
                C.ml_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__ml_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
            ]
        ],
        div![
            class![
                C.absolute,
                C.right_1of2,
                C.top_0,
                C.mt_790px,
                C.mr_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_1310px,
                C.sm__mr_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
            ]
        ],
        div![
            class![
                C.absolute,
                C.left_1of2,
                C.top_0,
                C.mt_1760px,
                C.ml_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_3040px,
                C.sm__ml_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
            ]
        ],
        div![
            class![
                C.absolute,
                C.right_1of2,
                C.top_0,
                C.mt_2340px,
                C.mr_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_3870px,
                C.sm__mr_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
            ]
        ],
    ]
}