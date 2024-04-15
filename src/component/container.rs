use super::Component;

pub trait Container: Sized + Component {
    fn with_child(mut self, mut child: impl Component) -> Self {
        child.set_depth(self.depth() - 1);
        child.apply_workspace(self.get_size(), self.get_position());
        self.children().unwrap().push(Box::new(child));
        self
    }
}
