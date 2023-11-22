use flow_wasm::app::Application;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Application>::new().render();
}
