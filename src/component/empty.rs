use super::{Component, Components, Container};

#[derive(Default)]
pub struct Empty {
    position: (f32, f32),
    size: (f32, f32),
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
}

impl Component for Empty {
    fn position_mut(&mut self) -> Option<(&mut f32, &mut f32)> {
        Some((&mut self.position.0, &mut self.position.1))
    }

    fn size_mut(&mut self) -> Option<(&mut f32, &mut f32)> {
        Some((&mut self.size.0, &mut self.size.1))
    }

    fn children(&mut self) -> Option<&mut super::Components> {
        Some(&mut self.children)
    }
}

impl Container for Empty {}
