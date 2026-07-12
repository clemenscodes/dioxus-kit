//! Headless page frame primitive: a route-level region stack that places the
//! [`Frame`]'s regions, stylable via the `attributes` extension. No modal behavior.

use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;

/// The headless page's props.
#[derive(Props, Clone, PartialEq)]
pub struct PageModel<F: Frame<Output = Element>> {
    /// The frame whose regions this page places.
    #[props(default)]
    pub frame: F,
    /// Styling for the page surface. Use `class:` at the call site.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A headless page frame: a region stack placing the frame's header, body, and footer.
#[component]
pub fn Page<F: Frame<Output = Element>>(props: PageModel<F>) -> Element {
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
        div {
            ..attributes,
            {header}
            {body}
            {footer}
        }
    }
}
