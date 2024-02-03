mod app;
mod config;
mod model;
mod rendering;
mod slideshow;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
