pub mod components;

use yew::prelude::*;

use crate::components::graph::Graph;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Graph></Graph>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
