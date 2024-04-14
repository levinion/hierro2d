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

    pub fn center(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                (*position.0, *position.1) = ((1. - size.0) / 2., (1. - size.1) / 2.);
            }
        }
        self
    }

    pub fn center_x(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                *position.0 = (1. - size.0) / 2.;
            }
        }
        self
    }

    pub fn center_y(mut self) -> Self {
        if let Some(size) = self.size_mut() {
            let size = (*size.0, *size.1);
            if let Some(position) = self.position_mut() {
                *position.1 = (1. - size.1) / 2.;
            }
        }
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
