use super::{Component, IntoComponent};

pub trait Container: Sized + Component {
    fn with_child(mut self, child: impl IntoComponent) -> Self {
        let mut child = child.into_comp();
        child.set_depth(self.depth() - 1);
        child.apply_workspace(self.get_size(), self.get_position());
        self.children().unwrap().push(child);
        self
    }
}
