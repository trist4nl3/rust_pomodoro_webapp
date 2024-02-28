mod components;
mod pages;
mod services;
use pages::home::Home;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Home>::new().render();
}