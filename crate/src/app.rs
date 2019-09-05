use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;
use crate::page;

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
    body()
        .set_attribute("class", &[
            C.font_body,
            C.text_18,
            C.bg_gray_1
        ].join(" "))
        .expect("cannot set body's class");

    document()
        .get_element_by_id("loading-page")
        .expect("cannot delete element with id 'loading-page'")
        .remove();

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
    page::home::view()
//    page::about::view()
}
