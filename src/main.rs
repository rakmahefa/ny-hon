mod app;
mod ui;

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(app::App);
}
