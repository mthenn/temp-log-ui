pub mod api;
pub mod components;

use std::ops::Sub;

use chrono::{Duration, Utc};
use yew::{prelude::*, props};

use crate::components::graph::Graph;
use crate::components::graph::GraphProps;

#[function_component]
fn App() -> Html {
    let from_date = { Utc::now() };
    let graph_props = props! {
        GraphProps {
            from_date: from_date,
            to_date: from_date.sub(Duration::days(4))
        }
    };
    html! {
        <div>
            <Graph
                ..graph_props
            />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
