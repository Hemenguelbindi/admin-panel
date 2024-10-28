mod app;
mod pages;
mod routes;
use app::AdminPanel;


fn main() {
    yew::Renderer::<AdminPanel>::new().render();
}