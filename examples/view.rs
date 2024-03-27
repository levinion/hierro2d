use hierro2d::{
    component::{Component, Square},
    Application,
};

struct App;

impl Application for App {
    fn view(&self) -> impl Into<Component> {
        Square::new().size(0.4, 0.5).position(0.1, 0.1)
    }
}

#[tokio::main]
async fn main() {
    let app = App {};
    app.run().await;
}
