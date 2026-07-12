//! Headless dialog frame primitive — structure + behavior, **zero look**.
//!
//! It wraps `dioxus_primitives`' `DialogRoot` (focus trap, escape / outside
//! dismiss — `DialogRoot` renders the dismiss overlay itself) and its
//! `DialogContent`, and places the [`Frame`]'s three regions directly inside
//! `DialogContent` — the element that carries `role="dialog"`. The consumer's
//! `class:` is forwarded onto `DialogContent`, so the `role="dialog"` element
//! itself is the styled, self-positioned container (no wrapper `div` inside it,
//! which would leave `role="dialog"` zero-sized when the content is positioned
//! `fixed`). A consuming app puts its own `classes!` `CLASS` there (a `ClassList`
//! is `IntoAttributeValue`, so `class:` works); this crate hand-builds no
//! attribute and carries no style of its own. The consumer's class owns the entire
//! look, positioning included.
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
    /// Styling for the `role="dialog"` content element. Use `class:` at the call
    /// site — it collects here through the attributes extension and is forwarded
    /// onto `DialogContent`. This crate adds no style of its own; the consumer's
    /// class owns the whole look, positioning included.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A headless dialog frame: `DialogRoot` behavior around the consumer-styled
/// `DialogContent` (`role="dialog"`) that places the frame's header, body, and footer.
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
                attributes,
                {header}
                {body}
                {footer}
            }
        }
    }
}
