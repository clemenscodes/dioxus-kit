//! Headless dialog frame primitive — structure + behavior, **zero look**.
//!
//! It wraps `dioxus_primitives`' `DialogRoot` (focus trap, escape / outside
//! dismiss — `DialogRoot` renders the dismiss overlay itself) and its
//! `DialogContent`, and places the [`Frame`]'s three regions inside a single
//! content container the consumer styles via `class:` — the one sanctioned way a
//! class flows. A consuming app puts its own `classes!` `CLASS` there (a
//! `ClassList` is `IntoAttributeValue`, so `class:` works); this crate hand-builds
//! no attribute and carries no style of its own. The consumer's class owns the
//! entire look, positioning included.
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
    /// Styling for the content container. Use `class:` at the call site — it
    /// collects here through the attributes extension. This crate adds no style of
    /// its own; the consumer's class owns the whole look.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A headless dialog frame: `DialogRoot` behavior around a single consumer-styled
/// content container that places the frame's header, body, and footer regions.
#[component]
pub fn Dialog<F: Frame<Output = Element>>(props: DialogModel<F>) -> Element {
    let frame = props.frame;
    let open = props.open;
    let on_open_change = props.on_open_change;
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
        DialogRoot {
            open,
            on_open_change,
            DialogContent {
                div {
                    ..attributes,
                    {header}
                    {body}
                    {footer}
                }
            }
        }
    }
}
