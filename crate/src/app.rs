use crate::generated::css_classes::C;
use crate::page;
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
    document()
        .get_element_by_id("loading-page")
        .expect("cannot find element with id `loading-page`")
        .remove();

    Model::default()
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {}
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class![
            C.fade_in,
        ],
        page::home::view()
//            page::about::view()
//            page::not_found::view()
        //    empty![]
    ]
}
