use hierro2d::{
    component::{Component, Container, Rect, Text},
    Application,
};

struct App;

impl Application for App {
    fn view(self) -> impl Component {
        let text = Text::new().content("Hello hierro2d!").depth(0).center();
        let sub_square = Rect::new()
            .size(0.5, 0.8)
            .center()
            .color(100, 100, 100, 0.5)
            .with_child(text)
            .depth(1);
        Rect::new()
            .size(0.9, 0.9)
            .position(0.1, 0.1)
            .radius(0.1)
            .with_child(sub_square)
            .depth(2)
    }

    fn window(&self, window: &mut winit::window::Window) {
        window.set_title("hello hierro2d");
    }
}

#[tokio::main]
async fn main() {
    let app = App {};
    app.run().await;
}
