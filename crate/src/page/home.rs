use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;

pub fn view<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        view_header().els(),
        view_content().els(),
        view_footer().els(),
    ].into_iter().flatten().collect()
}

pub fn view_header<Ms: 'static>() -> impl View<Ms> {
    header![
        "MK"
    ]
}

pub fn view_content<Ms: 'static>() -> impl View<Ms> {
    vec![
        // Main section
        section![
            div![
                h1![
                    class![
                        C.font_display
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
                    " is  a developer  with 7+ years of experience  who likes to design and ..."
                ]
            ],
            ul![
                class![
                    C.font_display
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
            h2![
                class![
                    C.font_display,
                    C.font_thin
                ],
                "TOP-5 GITHUB PROJECTS"
            ],
            div![
                div![
                    class![
                        C.font_display,
                        C.italic
                    ],
                    "Awesome, awesome framework!"
                ],
                div![
                    class![
                        C.font_display,
                    ],
                    "- rebo"
                ]
            ],
            div![
                div![
                    class![
                        C.font_display,
                        C.italic
                    ],
                    "Seed rocks, and Martin makes it better."
                ],
                div![
                    class![
                        C.font_display,
                    ],
                    "- robwebbjr"
                ]
            ],
            div![
                "Seed"
            ],
            ul![
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
                    "David-OConnor/"
                ],
                span![
                    "seed"
                ]
            ]
        ],
        // RealWorld section
        section![
            div![
                div![
                    class![
                        C.font_display,
                        C.italic
                    ],
                    "Your real world example really  is the mother of all examples."
                ],
                div![
                    class![
                        C.font_display,
                    ],
                    "- theduke"
                ]
            ],
            div![
                "RealWorld example app"
            ],
            ul![
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
                    "MartinKavik/"
                ],
                span![
                    "seed-rs-realworld"
                ]
            ]
        ],
        // Kavik.cz section
        section![
            div![
                div![
                    class![
                        C.font_display,
                        C.italic
                    ],
                    "Fork it, use it!"
                ],
                div![
                    class![
                        C.font_display,
                    ],
                    "- me"
                ]
            ],
            div![
                "MK"
            ],
            ul![
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
                    "MartinKavik/"
                ],
                span![
                    "kavik.cz"
                ]
            ]
        ],
        // Seed Quickstart section
        section![
            div![
                div![
                    class![
                        C.font_display,
                        C.italic
                    ],
                    "Its great!"
                ],
                div![
                    class![
                        C.font_display,
                    ],
                    "- rebo"
                ]
            ],
            div![
                "Seed Quickstart"
            ],
            ul![
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
                ],
                span![
                    "MartinKavik/"
                ],
                span![
                    "seed-quickstart-webpack"
                ]
            ]
        ],
        // Hellweb section
        section![
            div![
                "Hellweb"
            ],
            ul![
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
                    "MartinKavik/"
                ],
                span![
                    "hellweb-pain"
                ]
            ],
            div![
                span![
                    "About"
                ],
                " your new developer",
                div![
                    ">"
                ]
            ]
        ]
    ]
}

pub fn view_footer<Ms: 'static>() -> impl View<Ms> {
    footer![
        div![
            div![
                "MK",
                span![
                    class![
                        C.font_display,
                        C.font_semibold
                    ],
                    "2019"
                ]
            ],
            div![
                class![
                    C.font_display,
                    C.font_semibold
                ],
                "martin@kavik.cz"
            ],
            div![
                "^"
            ]
        ]
    ]
}

