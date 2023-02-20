use bitmask_enum::bitmask;

macro_rules! mouse_btn_confusion_doc {
    () => {
r#"
# WARNING

The order relation for [`MouseButtonId::Auxiliary`] and [`MouseButtonId::RightOrSecondary`]
is reversed for [`MouseButtonsBitmask::Auxiliary`] and [`MouseButtonsBitmask::RightOrSecondary`].
This behavior is preserved for compatibility with the
[`MouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent) API."#
    };
}

/// Button values for [`MouseEvent.button`] property.
///
#[doc = mouse_btn_confusion_doc!()]
///
/// [`MouseEvent.button`]: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MouseButtonId(pub i16);

#[allow(non_upper_case_globals)]
impl MouseButtonId {
    /// Left button or primary button.
    pub const LeftOrPrimary: Self = Self(0);
    /// Right button or secondary button.
    pub const RightOrSecondary: Self = Self(2);
    /// Mouse wheel button or middle button.
    pub const Auxiliary: Self = Self(1);
    /// Browser back button or 4th button.
    pub const BrowserBackOr4th: Self = Self(3);
    /// Browser forward button or 5th button.
    pub const BrowserForwardOr5th: Self = Self(4);
}

/// [Bitmask] [newtype] for [`MouseEvent.buttons`] property. In [`web_sys`],
/// this corresponds to the result of [`MouseEvent.buttons()`].
///
#[doc = mouse_btn_confusion_doc!()]
///
/// [Bitmask]: https://en.wikipedia.org/wiki/Mask_(computing)
/// [newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
/// [`MouseEvent.buttons`]: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
/// [`web_sys`]: https://crates.io/crates/web-sys
/// [`MouseEvent.buttons()`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.MouseEvent.html#method.buttons
#[bitmask(u16)]
pub enum MouseButtonsBitmask {
    /// Left button or primary button.
    LeftOrPrimary,
    /// Right button or secondary button.
    RightOrSecondary,
    /// Mouse wheel button or middle button.
    Auxiliary,
    /// Browser back button or 4th button.
    BrowserBackOr4th,
    /// Browser forward button or 5th button.
    BrowserForwardOr5th,
}

impl MouseButtonsBitmask {
    /// Creates a new bitmask from the given button value.
    /// 
    /// In [`web_sys`], this corresponds to the result of [`MouseEvent.button()`].
    /// 
    /// [`web_sys`]: https://crates.io/crates/web-sys
    /// [`MouseEvent.button()`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.MouseEvent.html#method.button
    pub fn new(buttons: u16) -> Self {
        // debug assert is not needed because there might be more buttons in the future
        MouseButtonsBitmask { bits: buttons }
    }
}

/// [Extension trait] for [`MouseEvent`].
/// 
/// [Extension trait]: https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html
/// [`MouseEvent`]: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent
pub trait MouseEventExt {
    /// Method-wrapper around [`MouseEvent.buttons`]. Unlike [`web-sys`] version,
    /// it returns a newtype instead of a raw integer.
    /// 
    /// [`MouseEvent.buttons`]: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
    /// [`web-sys`]: https://crates.io/crates/web-sys
    fn buttons_mask(&self) -> MouseButtonsBitmask;
    /// Method-wrapper around [`MouseEvent.button`]. Unlike [`web-sys`] version,
    /// it returns a newtype instead of a raw integer.
    /// 
    /// [`MouseEvent.button`]: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button
    /// [`web-sys`]: https://crates.io/crates/web-sys
    fn button_id(&self) -> MouseButtonId;
}

#[cfg(any(feature = "web-sys", doc))]
impl MouseEventExt for web_sys::MouseEvent {
    fn buttons_mask(&self) -> MouseButtonsBitmask {
        MouseButtonsBitmask::new(self.buttons())
    }

    fn button_id(&self) -> MouseButtonId {
        MouseButtonId(self.button())
    }
}

/// A trait for mouse button event handlers.
pub trait MouseButtonEventHandler {
    /// Handles a [`mousedown`] event.
    /// 
    /// [`mousedown`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
    fn handle_mousedown(&self);
    /// Handles a [`mouseup`] event.
    /// 
    /// [`mouseup`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
    fn handle_mouseup(&self);
    /// Handles a [`click`] event.
    /// 
    /// [`click`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event
    fn handle_click(&self);
    /// Handles a [`dblclick`] event.
    /// 
    /// [`dblclick`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event
    fn handle_dblclick(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weird_behavior_is_preserved() {
        assert!(MouseButtonsBitmask::Auxiliary.bits() == 4);
        assert!(MouseButtonsBitmask::RightOrSecondary.bits() == 2);
        assert!(MouseButtonsBitmask::Auxiliary.bits() > MouseButtonsBitmask::RightOrSecondary.bits());
        assert!(MouseButtonId::Auxiliary.0 == 1);
        assert!(MouseButtonId::RightOrSecondary.0 == 2);
        assert!(MouseButtonId::Auxiliary < MouseButtonId::RightOrSecondary);
    }
}
