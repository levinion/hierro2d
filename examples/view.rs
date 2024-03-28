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
            .text((255, 255, 255), "Hello Hierro2d!")
    }
}

#[tokio::main]
async fn main() {
    let app = App {};
    app.run().await;
}
