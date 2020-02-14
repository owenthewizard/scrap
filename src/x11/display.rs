use std::rc::Rc;
use super::Server;
use super::ffi::*;

#[derive(Debug)]
pub struct Display {
    server: Rc<Server>,
    default: bool,
    rect: Rect,
    root: xcb_window_t,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

impl Display {
    #[must_use]
    pub unsafe fn new(
        server: Rc<Server>,
        default: bool,
        rect: Rect,
        root: xcb_window_t
    ) -> Self {
        Self { server, default, rect, root }
    }

    #[must_use]
    pub const fn server(&self) -> &Rc<Server> { &self.server }

    #[must_use]
    pub const fn is_default(&self) -> bool { self.default }

    #[must_use]
    pub const fn rect(&self) -> Rect { self.rect }

    #[must_use]
    pub const fn root(&self) -> xcb_window_t { self.root }
}
