//! The no-op region: a [`Render`] that renders nothing, for a [`Frame`] that omits
//! its header or footer.

use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published view contract for [`Empty`] (carries nothing).
#[derive(Clone, PartialEq, Default)]
pub struct EmptyView;

impl ddd::Layered for EmptyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyView {}

/// The internal model for [`Empty`] (carries nothing).
#[derive(Clone, PartialEq, Default)]
pub struct EmptyModel;

impl ddd::Layered for EmptyModel {
    type Layer = ddd::PresentationLayer;
}

impl From<&EmptyView> for EmptyModel {
    fn from(_view: &EmptyView) -> Self {
        Self
    }
}

impl ddd::Model for EmptyModel {
    type View = EmptyView;
}

/// A region that renders nothing — the default `Header`/`Footer` type of a [`Frame`]
/// that has none.
///
/// [`Frame`]: browser_kit::frame::Frame
#[derive(Clone, PartialEq, Default)]
pub struct Empty;

impl Render for Empty {
    type Model = EmptyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        rsx! {}
    }
}
