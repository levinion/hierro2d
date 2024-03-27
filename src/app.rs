use std::future::Future;

use crate::{component::Component, winit};

pub trait Application: Sized {
    fn view(&self) -> impl Into<Component>;

    fn run(self) -> impl Future<Output = ()> {
        winit::run(self)
    }
}
