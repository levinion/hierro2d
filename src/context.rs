use winit::window::Fullscreen;

use crate::state::State;

pub struct Context<'a>(pub(crate) &'a mut State);

impl<'a> Context<'a> {
    pub fn set_fullscreen(&self, fullscreen: bool) {
        match fullscreen {
            true => self
                .0
                .window()
                .set_fullscreen(Some(Fullscreen::Borderless(None))),
            false => self.0.window.set_fullscreen(None),
        }
    }

    pub fn toggle_fullscreen(&self) {
        match self.0.window().fullscreen() {
            Some(_) => self.0.window().set_fullscreen(None),
            None => self
                .0
                .window()
                .set_fullscreen(Some(Fullscreen::Borderless(None))),
        }
    }
}
