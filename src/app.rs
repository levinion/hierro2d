use std::future::Future;

use crate::component::Component;

pub trait Application: Sized + 'static {
    fn view(self) -> impl Component;

    fn run(self) -> impl Future<Output = ()> {
        crate::winit::run(self)
    }

    fn window(&self, _window: &mut winit::window::Window) {}
}
