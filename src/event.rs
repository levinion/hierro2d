use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::{
    component::{Comp, Component},
    state::State,
};

impl State {
    fn get_element_by_pos(&self) -> Option<&Comp> {
        self.components.iter().rfind(|x| {
            let position = x.get_position();
            let size = x.get_size();
            self.cursor_pos.0 >= position.0 as f64
                && self.cursor_pos.0 < position.0 as f64 + size.0 as f64
                && self.cursor_pos.1 <= position.1 as f64
                && self.cursor_pos.1 > position.1 as f64 - size.1 as f64
        })
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        #[allow(clippy::collapsible_match)]
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if let MouseButton::Left = button {
                    if let ElementState::Pressed = state {
                        if let Some(comp) = self.get_element_by_pos() {
                            if let Some(f) = comp.click_handler() {
                                f(self.as_ctx());
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let size = (self.size().width as f64, self.size().height as f64);
                self.cursor_pos = (
                    position.x / size.0 * 2. - 1.,
                    -position.y / size.1 * 2. + 1.,
                );
            }
            _ => {}
        }
        false
    }
}
