mod model;
mod rendering;
mod components;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<components::app::App>::new().render();
}
