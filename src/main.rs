mod components;
mod pages;
use pages::home::Home;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Home>::new().render();
}