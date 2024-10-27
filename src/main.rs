mod app;
use app::AdminPanel;


fn main() {
    yew::Renderer::<AdminPanel>::new().render();
}