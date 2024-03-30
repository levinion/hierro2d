use hierro2d::{
    component::{Component, Container, Empty, Rect, Text},
    Application,
};

struct App;

impl Application for App {
    fn view(self) -> impl Component {
        let text = Text::new().content("Hello hierro2d!").depth(0);
        let sub_square = Rect::new()
            .size(0.8, 0.8)
            .position(0.1, 0.1)
            .color(100, 100, 100, 0.5)
            .with_child(text)
            .depth(1);
        let square = Rect::new()
            .size(0.9, 0.9)
            .position(0.05, 0.05)
            .radius(0.1)
            .with_child(sub_square)
            .depth(2);

        Empty::new()
            .position(0.05, 0.05)
            .size(0.9, 0.9)
            .with_child(square)
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
