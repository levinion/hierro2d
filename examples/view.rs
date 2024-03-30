use hierro2d::{
    component::{Component, Container, Square, Text},
    Application,
};

struct App;

impl Application for App {
    fn view(self) -> impl Component {
        let text = Text::new().content("Hello hierro2d!").depth(0);
        let sub_square = Square::new()
            .size(0.8, 0.8)
            .position(0.1, 0.1)
            .color(100, 100, 100)
            .with_child(text)
            .depth(1);
        Square::new()
            .size(0.4, 0.4)
            .position(0.1, 0.1)
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
