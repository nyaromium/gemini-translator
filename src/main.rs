mod gui;
mod screen_capture;
mod api;

#[tokio::main]
async fn main() {
    gui::main();
    screen_capture::main();
    api::main().await.expect("TODO: panic message");
}