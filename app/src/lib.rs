#![recursion_limit = "4096"]
use index::IndexModel;
use initial::InitialModel;
use new_post::NewPostModel;
use post::PostModel;
use serde::Deserialize;
use sign_in::SignInModel;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

#[derive(cynic::Scalar, Deserialize)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(cynic::Scalar, Deserialize)]
pub struct Jsonobject(serde_json::value::Value);

mod index;
mod initial;
mod new_post;
mod post;
mod sign_in;

struct Model {}

pub mod query_dsl {
    use crate::{DateTime, Jsonobject};

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
                    AppRoute::Index => html!{<IndexModel />},
                    AppRoute::Post(id) => html!{<PostModel id={id} />},
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
    #[to = "/posts/new"]
    NewPost,
    #[to = "/post/{id}"]
    Post(i32),
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
