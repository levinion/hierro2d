use hierro2d::{
    component::{Component, Container, Img, IntoComponent, Rect, Text},
    Application,
};

struct App;

impl Application for App {
    fn view(self) -> impl IntoComponent {
        let img = Img::new()
            .content("/home/maruka/Pictures/bg.jpg")
            .unwrap()
            .size(0.1, 0.1);
        let text = Text::new()
            .content("Hello hierro2d!")
            .center()
            .on_click(|ctx| ctx.toggle_fullscreen());
        let sub_rect = Rect::new()
            .size(0.2, 0.2)
            .center()
            .color(100. / 255., 100. / 255., 100. / 255., 0.5)
            .with_child(text);
        Rect::new()
            .size(0.9, 0.9)
            .position(0.1, 0.1)
            .radius(0.1)
            .with_child(sub_rect)
            .with_child(img)
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
