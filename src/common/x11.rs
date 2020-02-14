use x11;
use std::{io, ops};
use std::rc::Rc;

pub struct Capturer(x11::Capturer);

impl Capturer {
    pub fn new(display: Display) -> io::Result<Self> {
        x11::Capturer::new(display.0).map(Self)
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.0.display().rect().w as usize
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.0.display().rect().h as usize
    }

    pub fn frame<'a>(&'a mut self) -> io::Result<Frame<'a>> {
        Ok(Frame(self.0.frame()))
    }
}

pub struct Frame<'a>(&'a mut [u8]);

impl<'a> ops::Deref for Frame<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> ops::DerefMut for Frame<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Display(x11::Display);

impl Display {
    pub fn primary() -> io::Result<Self> {
        let server = Rc::new(match x11::Server::default() {
            Ok(server) => server,
            Err(_) => return Err(io::ErrorKind::ConnectionRefused.into())
        });

        let mut displays = x11::Server::displays(server);
        let mut best = displays.next();
        if best.as_ref().map(|x| x.is_default()) == Some(false) {
            best = displays.find(|x| x.is_default()).or(best);
        }

        match best {
            Some(best) => Ok(Self(best)),
            None => Err(io::ErrorKind::NotFound.into())
        }
    }

    pub fn all() -> io::Result<Vec<Self>> {
        let server = Rc::new(match x11::Server::default() {
            Ok(server) => server,
            Err(_) => return Err(io::ErrorKind::ConnectionRefused.into())
        });

        Ok(x11::Server::displays(server).map(Self).collect())
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.0.rect().w as usize
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.0.rect().h as usize
    }

    #[must_use]
    pub fn top(&self) -> i32 {
        i32::from(self.0.rect().y)
    }

    #[must_use]
    pub fn bottom(&self) -> i32 {
        i32::from(self.0.rect().y + self.0.rect().h as i16)
    }

    #[must_use]
    pub fn left(&self) -> i32 {
        i32::from(self.0.rect().x)
    }

    #[must_use]
    pub fn right(&self) -> i32 {
        i32::from(self.0.rect().x + self.0.rect().w as i16)
    }
}
