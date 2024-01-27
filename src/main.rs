mod components;
mod model;
mod rendering;

use components::Slideshow;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Slideshow>::new().render();
}
