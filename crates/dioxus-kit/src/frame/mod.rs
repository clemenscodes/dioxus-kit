//! The Frame system's Dioxus primitives: headless frame components built on the
//! framework-agnostic `browser_kit::frame::{Render, Frame}` contracts.
//!
//! Headless — structure and behavior only, no look. A consuming app styles a frame
//! by passing its own class through the `attributes` extension (Radix-for-dioxus).

mod card;
mod dialog;
mod empty;
mod page;

pub use card::{Card, CardModel};
pub use dialog::{Dialog, DialogModel};
pub use empty::{Empty, EmptyModel, EmptyView};
pub use page::{Page, PageModel};
