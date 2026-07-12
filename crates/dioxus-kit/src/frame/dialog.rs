//! Headless dialog frame primitive.
//!
//! Structure + behavior, no look: it wraps `dioxus_primitives`' `DialogRoot`
//! (focus trap, escape / outside dismiss) and its `DialogContent`, places the
//! [`Frame`]'s three regions, and exposes two stylable parts — the **backdrop**
//! (the outer surface, via the `attributes` extension so `class:` works) and the
//! **panel** (the inner box, via `panel_attributes`). A consuming app styles both
//! from its own `classes!` and never sees this crate's internals.
//!
//! Body scroll-lock is intentionally not owned here yet (a follow-up); `DialogRoot`
//! already provides focus trap, escape, and outside-dismiss.

use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

/// The headless dialog's props.
#[derive(Props, Clone, PartialEq)]
pub struct DialogModel<F: Frame<Output = Element>> {
    /// The frame whose regions this dialog places.
    #[props(default)]
    pub frame: F,
    /// Whether the dialog is open.
    pub open: bool,
    /// Fired when the open state changes (escape, outside click, programmatic).
    pub on_open_change: Callback<bool>,
    /// Styling for the **backdrop** part (the outer surface). Use `class:` at the
    /// call site — it collects here through the attributes extension.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// Styling for the **panel** part (the inner box).
    #[props(default)]
    pub panel_attributes: Vec<Attribute>,
}

/// A headless dialog frame: `DialogRoot` behavior + a backdrop and a panel part,
/// placing the frame's header, body, and footer regions.
#[component]
pub fn Dialog<F: Frame<Output = Element>>(props: DialogModel<F>) -> Element {
    let frame = props.frame;
    let open = props.open;
    let on_open_change = props.on_open_change;
    let attributes = props.attributes;
    let panel_attributes = props.panel_attributes;
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
        DialogRoot {
            open,
            on_open_change,
            div {
                ..attributes,
                DialogContent {
                    div {
                        ..panel_attributes,
                        {header}
                        {body}
                        {footer}
                    }
                }
            }
        }
    }
}
