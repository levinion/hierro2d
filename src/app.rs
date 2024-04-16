use std::future::Future;

use crate::component::IntoComponent;

pub trait Application: Sized + 'static {
    fn view(self) -> impl IntoComponent;

    fn run(self) -> impl Future<Output = ()> {
        crate::backend::run(self)
    }

    fn window(&self, _window: &mut winit::window::Window) {}
}
