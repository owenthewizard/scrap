use std::ptr;
use std::rc::Rc;
use super::DisplayIter;
use super::ffi::*;

#[derive(Debug)]
pub struct Server {
    raw: *mut xcb_connection_t,
    screenp: i32,
    setup: *const xcb_setup_t
}

impl Server {
    #[must_use]
    pub fn displays(slf: Rc<Self>) -> DisplayIter {
        unsafe {
            DisplayIter::new(slf)
        }
    }

    pub fn default() -> Result<Self, Error> {
        Self::connect(ptr::null())
    }

    pub fn connect(addr: *const i8) -> Result<Self, Error> {
        unsafe {
            let mut screenp = 0;
            // may be unsafe, see clippy lint
            let raw = xcb_connect(addr, &mut screenp);

            let error = xcb_connection_has_error(raw);
            if error == 0 {
                let setup = xcb_get_setup(raw);
                Ok(Self { raw, screenp, setup })
            } else {
                xcb_disconnect(raw);
                Err(Error::from(error))
            }
        }
    }

    #[must_use]
    pub const fn raw(&self) -> *mut xcb_connection_t { self.raw }

    #[must_use]
    pub const fn screenp(&self) -> i32 { self.screenp }

    #[must_use]
    pub const fn setup(&self) -> *const xcb_setup_t { self.setup }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            xcb_disconnect(self.raw);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Generic,
    UnsupportedExtension,
    InsufficientMemory,
    RequestTooLong,
    ParseError,
    InvalidScreen
}

impl From<i32> for Error {
    fn from(x: i32) -> Self {
        use self::Error::*;
        match x {
            2 => UnsupportedExtension,
            3 => InsufficientMemory,
            4 => RequestTooLong,
            5 => ParseError,
            6 => InvalidScreen,
            _ => Generic
        }
    }
}
