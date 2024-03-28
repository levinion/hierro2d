use hierro2d::{
    component::{IntoComponent, Square},
    Application,
};

struct App;

impl Application for App {
    fn view(&self) -> impl IntoComponent {
        Square::new()
            .size(0.4, 0.4)
            .position(0.1, 0.1)
            .with_text(|text| {
                text.set_content("Hello hierro2d!\n你好，hierro2d！");
            })
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
