#![recursion_limit="4096"]
use index::IndexModel;
use sign_in::SignInModel;
use initial::InitialModel;
use post::PostModel;
use new_post::NewPostModel;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};
use serde::Deserialize;

#[derive(cynic::Scalar, Deserialize)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

mod index;
mod post;
mod initial;
mod sign_in;
mod new_post;

struct Model {}

pub mod query_dsl {
    use crate::DateTime;

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
                    AppRoute::Initial => html!{<InitialModel />},
                    AppRoute::SignIn => html!{<SignInModel />},
                    AppRoute::NewPost => html!{<NewPostModel />},
                }
            })
        />
            }
    }
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/posts_new"]
    NewPost,
    #[to = "/not_index"]
    NotIndex,
    #[to = "/post"]
    Post,
    #[to = "/initial"]
    Initial,
    #[to = "/sign/in"]
    SignIn,
    #[to = "/"]
    Index,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
