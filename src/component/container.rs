use super::Component;

pub trait Container: Sized {
    fn with_child(self, child: impl Component) -> Self;
}
