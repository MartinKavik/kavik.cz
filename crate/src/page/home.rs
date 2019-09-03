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
                    span![
                        "Martin "
                    ],
                    span![
                        "Kavík"
                    ],
                ],
                span![
                    " is  a developer  with 7+ years of experience  who likes to design and ..."
                ]
            ],
            ul![
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
                        "martin@kavik.cz"
                    ]
                ],
            ]
        ],
        // Seed section
        section![
            h2![
                "TOP-5 GITHUB PROJECTS"
            ],
            div![
                div![
                    "Awesome, awesome framework!"
                ],
                div![
                    "- rebo"
                ]
            ],
            div![
                div![
                    "Seed rocks, and Martin makes it better."
                ],
                div![
                    "- robwebbjr"
                ]
            ],
            div![
                "Seed"
            ],
            ul![
                li![
                    h3![
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
                    "Your real world example really  is the mother of all examples."
                ],
                div![
                    "- theduke"
                ]
            ],
            div![
                "RealWorld example app"
            ],
            ul![
                li![
                    h3![
                        "RealWorld example"
                    ],
                    " is a Seed codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the RealWorld spec and API."
                ],
            ],
            div![
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
                    "Fork it, use it!"
                ],
                div![
                    "- me"
                ]
            ],
            div![
                "MK"
            ],
            ul![
                li![
                    h3![
                        "kavik.cz"
                    ],
                    " is this website."
                ],
                li![
                    "You can fork it, modify it and use it as your own website."
                ],
            ],
            div![
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
                    "Its great!"
                ],
                div![
                    "- rebo"
                ]
            ],
            div![
                "Seed Quickstart"
            ],
            ul![
                li![
                    h3![
                        "Seed Quickstart"
                    ],
                    " is a template for web apps with Seed, TailwindCSS, Typescript and Webpack."
                ],
            ],
            div![
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
                        "Hellweb"
                    ],
                    " will be a collection of Rust libraries and applications which solve your pain points and explore new ideas."
                ],
                li![
                    "What do you hate or what annoys you about ",
                    span![
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

//    vec![
//        h1![
//           "Home"
//        ],
//        h1![
//            style! {
//                "margin" => "21px",
//            },
//            class! [
//                C.font_sans
//            ],
//            "Martin Kavík"
//        ],
//        h2![
//            style! {
//                "margin" => "21px",
//            },
//            class! [
//                C.font_sans
//            ],
//            "martin@kavik.cz"
//        ]
//    ]
}

pub fn view_footer<Ms: 'static>() -> impl View<Ms> {
    footer![
        div![
            div![
                "MK",
                span![
                    "2019"
                ]
            ],
            div![
                "martin@kavik.cz"
            ],
            div![
                "^"
            ]
        ]
    ]
}

