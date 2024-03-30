use super::Component;

pub trait Container: Sized + Component {
    fn with_child(mut self, mut child: impl Component) -> Self {
        let (x, y) = self.position_mut().unwrap();
        let (x, y) = (*x, *y);
        let (width, height) = self.size_mut().unwrap();
        let (width, height) = (*width, *height);
        child.apply_workspace((width, height), (x, y));
        self.children().unwrap().push(Box::new(child));
        self
    }
}
