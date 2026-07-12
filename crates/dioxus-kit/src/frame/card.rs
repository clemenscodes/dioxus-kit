//! Headless card frame primitive: a static container that places the [`Frame`]'s
//! regions, stylable via the `attributes` extension. No modal behavior.

use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;

/// The headless card's props.
#[derive(Props, Clone, PartialEq)]
pub struct CardModel<F: Frame<Output = Element>> {
    /// The frame whose regions this card places.
    #[props(default)]
    pub frame: F,
    /// Styling for the card container. Use `class:` at the call site.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A headless card frame: a container placing the frame's header, body, and footer.
#[component]
pub fn Card<F: Frame<Output = Element>>(props: CardModel<F>) -> Element {
    let frame = props.frame;
    let attributes = props.attributes;
    let body = frame.body().render();
    let header = match frame.header() {
        Some(region) => region.render(),
        None => rsx! {},
    };
    let footer = match frame.footer() {
        Some(region) => region.render(),
        None => rsx! {},
    };
    rsx! {
        section {
            ..attributes,
            {header}
            {body}
            {footer}
        }
    }
}
