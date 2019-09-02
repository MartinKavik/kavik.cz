use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;

// ------ ------
//     Model
// ------ ------

pub struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {

}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
    vec![
        h1![
            style! {
                "margin" => "21px",
            },
            class! [
                C.font_sans
            ],
            "Martin Kavík"
        ],
        h2![
            style! {
                "margin" => "21px",
            },
            class! [
                C.font_sans
            ],
            "martin@kavik.cz"
        ]
    ]
}
