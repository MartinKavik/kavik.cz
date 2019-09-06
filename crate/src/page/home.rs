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
        // Main section
        section![
            div![
                class![
                    C.font_display,
                ],
                h1![
                    class![
                        C.text_31,
                        C.text_gray_10
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
                        C.text_gray_7
                    ],
                    " is  a developer  with 7+ years of experience  who likes to design and ..."
                ]
            ],
            ul![
                class![
                    C.font_display,
                    C.text_17,
                    C.text_gray_8
                ],
                li![
                    "To work on your project"
                ],
                li![
                    "Readable code and UI"
                ],
                li![
                    "Rust, Affinity Designer and Figma."
                ],
                li![
                    "Receiving mails. ",
                    span![
                        class![
                            C.font_semibold
                        ],
                        "martin@kavik.cz"
                    ]
                ],
            ]
        ],
        // Seed section
        section![
            class![
                C.bg_blue_6
            ],
            h2![
                class![
                    C.font_display,
                    C.font_thin,
                    C.text_23,
                    C.text_blue_3
                ],
                "TOP-5 GITHUB PROJECTS"
            ],
            div![
                div![
                    class![
                        C.font_display,
                        C.italic,
                        C.text_16,
                        C.text_yellow_4
                    ],
                    "Awesome, awesome framework!"
                ],
                div![
                    class![
                        C.font_display,
                        C.text_15,
                        C.text_blue_3
                    ],
                    "- rebo"
                ]
            ],
            div![
                div![
                    class![
                        C.font_display,
                        C.italic,
                        C.text_16,
                        C.text_yellow_4
                    ],
                    "Seed rocks, and Martin makes it better."
                ],
                div![
                    class![
                        C.font_display,
                        C.text_15,
                        C.text_blue_3
                    ],
                    "- robwebbjr"
                ]
            ],
            div![
                class![
                    C.font_display,
                    C.text_70,
                    C.font_semibold
                ],
                "Seed"
            ],
            ul![
                class![
                    C.text_blue_1
                ],
                li![
                    h3![
                        class![
                            C.font_bold,
                        ],
                        "Seed"
                    ],
                    " is an open-source Rust framework for creating fast and reliable web apps running in WebAssembly."
                ],
                li![
                    "I'm the main contributor."
                ],
                li![
                    "I've designed the logo."
                ],
            ],
            div![
                class![
                    C.font_display,
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
                ]
            ]
        ],
        // RealWorld section
        section![
            class![
                C.bg_blue_10
            ],
            div![
                div![
                    class![
                        C.font_display,
                        C.italic,
                        C.text_yellow_4
                    ],
                    "Your real world example really  is the mother of all examples."
                ],
                div![
                    class![
                        C.font_display,
                        C.text_15,
                        C.text_blue_3
                    ],
                    "- theduke"
                ]
            ],
            div![
                class![
                    C.font_display,
                    C.text_20,
                    C.font_bold
                ],
                "RealWorld example app"
            ],
            ul![
                class![
                    C.text_blue_1
                ],
                li![
                    h3![
                        class![
                            C.font_bold,
                        ],
                        "RealWorld example"
                    ],
                    " is a Seed codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the RealWorld spec and API."
                ],
            ],
            div![
                class![
                    C.font_display,
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
                ]
            ]
        ],
        // Kavik.cz section
        section![
            class![
                C.bg_blue_6
            ],
            div![
                div![
                    class![
                        C.font_display,
                        C.italic,
                        C.text_yellow_4
                    ],
                    "Fork it, use it!"
                ],
                div![
                    class![
                        C.font_display,
                        C.text_15,
                        C.text_blue_3
                    ],
                    "- me"
                ]
            ],
            div![
                class![
                    C.font_display,
                    C.text_80,
                    C.font_bold
                ],
                "MK"
            ],
            ul![
                class![
                    C.text_blue_1
                ],
                li![
                    h3![
                        class![
                            C.font_bold,
                        ],
                        "kavik.cz"
                    ],
                    " is this website."
                ],
                li![
                    "You can fork it, modify it and use it as your own website."
                ],
            ],
            div![
                class![
                    C.font_display,
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
                ]
            ]
        ],
        // Seed Quickstart section
        section![
            class![
                C.bg_blue_10
            ],
            div![
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
                        C.font_display,
                        C.text_15,
                        C.text_blue_3
                    ],
                    "- rebo"
                ]
            ],
            div![
                class![
                    C.font_display,
                    C.font_bold,
                    C.text_34
                ],
                "Seed Quickstart"
            ],
            ul![
                class![
                    C.text_blue_1
                ],
                li![
                    h3![
                        class![
                            C.font_bold,
                        ],
                        "Seed Quickstart"
                    ],
                    " is a template for web apps with Seed, TailwindCSS, Typescript and Webpack."
                ],
            ],
            div![
                class![
                    C.font_display,
                    C.text_15
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
                ]
            ]
        ],
        // Hellweb section
        section![
            div![
                class![
                    C.font_display,
                    C.font_bold,
                    C.text_45,
                    C.text_yellow_10
                ],
                "Hellweb"
            ],
            ul![
                class![
                    C.text_gray_10
                ],
                li![
                    h3![
                        class![
                            C.font_bold,
                        ],
                        "Hellweb"
                    ],
                    " will be a collection of Rust libraries and applications which solve your pain points and explore new ideas."
                ],
                li![
                    "What do you hate or what annoys you about ",
                    span![
                        class![
                            C.font_bold,
                        ],
                        "web design & development"
                    ],
                    " ? Don't hesitate to create an issue or contact me - ",
                    span![
                        "martin@kavik.cz"
                    ]
                ]
            ],
            div![
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
                ]
            ],
            div![
                class![
                    C.text_19,
                    C.text_gray_10
                ],
                span![
                    "About"
                ],
                " your new developer",
                div![
                    class![
                        C.text_yellow_6
                    ],
                    ">"
                ]
            ]
        ]
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
