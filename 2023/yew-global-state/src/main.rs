mod layout;
mod model;

use crate::layout::AppLayout;

fn main() {
    yew::Renderer::<AppLayout>::new().render();
}
