use yew::prelude::*;

mod app;
mod utils;
mod layouts;
mod components;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
