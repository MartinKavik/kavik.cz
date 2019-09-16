use std::convert::TryFrom;

// ------ Route ------

#[derive(Clone)]
pub enum Route {
    Home,
    About,
    Redirect,
}

impl<'a> TryFrom<seed::Url> for Route {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        // Seed doesn't recognize files as external links
        if url.path.starts_with(&["static".into()]) {
            seed::window().location().assign(&format!("/{}", url.path.join("/")))
                .expect("cannot change location");
        }

        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("about") => Some(Route::About),
            Some("static") => Some(Route::Redirect),
            _ => None,
        }
            .ok_or(())
    }
}