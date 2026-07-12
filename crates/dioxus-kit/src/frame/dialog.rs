//! Headless dialog frame primitive.
//!
//! Structure + behavior: it wraps `dioxus_primitives`' `DialogRoot` (focus trap,
//! escape / outside dismiss) and its `DialogContent`, and places the [`Frame`]'s
//! three regions inside a **content container** the consumer styles via `class:`
//! (the standard attributes extension — the one sanctioned way a class flows). A
//! consuming app puts its own `classes!` `CLASS` there and never hand-builds an
//! attribute.
//!
//! The dialog owns a neutral **structural** backdrop (fixed, centred, a plain
//! scrim). A modal needs one, and the consumer's single `class:` is spent on the
//! content — so the app-specific look lives entirely on the content container while
//! the backdrop stays a look-agnostic positioner. Consumers never style it.
//!
//! Body scroll-lock is intentionally not owned here yet (a follow-up); `DialogRoot`
//! already provides focus trap, escape, and outside-dismiss.

use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

/// The neutral structural backdrop: fixed, centred, a plain scrim. Not app-specific
/// look — the consumer styles the content container, never this positioner.
const BACKDROP_STYLE: &str =
    "position:fixed;inset:0;display:grid;place-items:center;background:rgba(0,0,0,0.5)";

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
    /// Styling for the **content container** (the inner box that holds the regions).
    /// Use `class:` at the call site — it collects here through the attributes
    /// extension. The backdrop is owned by the dialog and is not styled here.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A headless dialog frame: `DialogRoot` behavior + a neutral backdrop wrapping a
/// consumer-styled content container that places the frame's header, body, and footer.
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
            div {
                style: BACKDROP_STYLE,
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
}
