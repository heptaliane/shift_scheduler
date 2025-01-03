use yew::Renderer;

mod app;
mod ga;
mod scheduler;

fn main() {
    Renderer::<app::App>::new().render();
}
