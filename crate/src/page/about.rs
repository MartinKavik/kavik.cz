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
        // Photo section
        section![
            div![
                "me"
            ],
        ],
        // Developer section
        section![
            h2![
                class![
                    C.font_monospace,
                    C.font_bold,
                    C.text_40
                ],
                "I, developer"
            ],
            ul![
                li![
                    "I was working as a ",
                    span![
                        class![
                            C.font_bold,
                        ],
                        "backend"
                    ],
                    " developer in a bank and for some startups and agencies last years."
                ],
                li![
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
                ],
                li![
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
                ],
                li![
                    "I often learn from ",
                    span![
                        "packtpub.com"
                    ],
                    ".  And I recommend to read book ",
                    span![
                        "Domain Modeling Made Functional"
                    ],
                    "."
                ],
            ]
        ],
        // Designer section
        section![
            h2![
                class![
                    C.font_display,
                    C.font_bold,
                    C.text_40
                ],
                "I, designer"
            ],
            ul![
                li![
                    "I've designed logos, my resume and this website in ",
                    span![
                        class![
                            C.font_bold,
                        ],
                        "Affinity Designer"
                    ],
                    "."
                ],
                li![
                    "I'll use ",
                    span![
                        class![
                            C.font_bold,
                        ],
                        "Figma"
                    ],
                    " for my next website design."
                ],
                li![
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
                ],
                li![
                    "I recommend to check ",
                    span![
                        "refactoringui.com"
                    ],
                    ". I've bought their book and I use their ",
                    span![
                        "TailwindCSS"
                    ],
                    " in my projects."
                ],
            ]
        ],
        // Human section
        section![
            h2![
                class![
                    C.font_ordinary,
                    C.font_bold,
                    C.text_40
                ],
                "I, human"
            ],
            div![
                h3![
                    class![
                        C.font_display,
                        C.font_thin,
                        C.text_35
                    ],
                    "Personal life"
                ],
                ul![
                    li![
                        "I'm INTJ. When I'm not ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "creating"
                        ],
                        " something, I usually read or go to gym."
                    ],
                    li![
                        "I like to spend my vacation at the cottage - hiking, cycling, driving, etc."
                    ]
                ]
            ],
            div![
                h3![
                    class![
                        C.font_display,
                        C.font_thin,
                        C.text_35
                    ],
                    "Work life"
                ],
                ul![
                    li![
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
                    ],
                    li![
                        "I like to ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "help"
                        ],
                        " people (not only on GitHub) and to mentor juniors."
                    ],
                    li![
                        "I'd rather think about your project for free in a gym than sit and wait for ideas. I also recommend to read ",
                        span![
                            "You don’t need standup"
                        ],
                        "."
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
                    C.text_40
                ],
                "Did you know"
            ],
            ul![
                li![
                    "I programmed a real football cannon."
                ],
                li![
                    "I jumped off a plane and a bridge."
                ],
            ]
        ],
        // Want to meet section
        section![
            ul![
                li![
                    "Want to meet somewhere in ",
                    span![
                        class![
                            C.font_bold
                        ],
                        "Prague"
                    ],
                    "?  Is there good coffee, tea, sushi or some spicy food? Ok! ",
                    span![
                        "martin@kavik.cz"
                    ]
                ],
            ]
        ],
        // Resume section
        section![
            div![
                class![
                    C.text_19
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
                        C.font_semibold
                    ],
                    ".pdf"
                ]
            ]
        ],
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
                        C.font_semibold,
                        C.text_15
                    ],
                    "2019"
                ]
            ],
            div![
                class![
                    C.font_display,
                    C.font_semibold,
                    C.text_16
                ],
                "martin@kavik.cz"
            ],
            div![
                "^"
            ]
        ]
    ]
}

