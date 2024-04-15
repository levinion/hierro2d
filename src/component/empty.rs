use super::{Component, Components, Container};

#[derive(Default)]
pub struct Empty {
    position: (f32, f32),
    size: (f32, f32),
    depth: i32,
    children: Components,
}

impl Empty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn center(mut self) -> Self {
        let size = self.get_size();
        self.set_position(((1. - size.0) / 2., (1. - size.1) / 2.));
        self
    }

    pub fn center_x(mut self) -> Self {
        let size = self.get_size();
        let position = self.get_position();
        self.set_position(((1. - size.0) / 2., position.1));
        self
    }

    pub fn center_y(mut self) -> Self {
        let size = self.get_size();
        let position = self.get_position();
        self.set_position((position.0, (1. - size.1) / 2.));
        self
    }
}

impl Component for Empty {
    fn children(&mut self) -> Option<&mut super::Components> {
        Some(&mut self.children)
    }

    fn depth(&self) -> i32 {
        self.depth
    }

    fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }

    fn get_position(&self) -> (f32, f32) {
        self.position
    }

    fn set_position(&mut self, position: (f32, f32)) {
        self.position = position;
    }

    fn get_size(&self) -> (f32, f32) {
        self.size
    }

    fn set_size(&mut self, size: (f32, f32)) {
        self.size = size;
    }
}

impl Container for Empty {}
