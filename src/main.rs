mod components;
mod hooks;
mod models;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
