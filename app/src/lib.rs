#![recursion_limit="4096"]
use index::IndexModel;
use post::PostModel;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

mod index;
mod post;

struct Model {}

pub mod query_dsl {
    cynic::query_dsl!("schema.graphql");
}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
        <Router<AppRoute, ()>
            render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::NotIndex => html!{<h1>{"Not Index"}</h1>},
                    AppRoute::Index => html!{<IndexModel />},
                    AppRoute::Post => html!{<PostModel />},
                }
            })
        />
            }
    }
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/not_index"]
    NotIndex,
    #[to = "/post"]
    Post,
    #[to = "/"]
    Index,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
