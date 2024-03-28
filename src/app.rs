use std::future::Future;

use crate::{component::IntoComponent, winit};

pub trait Application: Sized {
    fn view(&self) -> impl IntoComponent;

    fn run(self) -> impl Future<Output = ()> {
        winit::run(self)
    }
}
