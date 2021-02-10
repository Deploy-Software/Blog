#![recursion_limit = "2048"]
use cynic::GraphQLResponse;
use cynic::QueryBuilder;
use index::IndexModel;
use initial::InitialModel;
use new_post::NewPostModel;
use post::PostModel;
use serde::Deserialize;
use serde_json::{value::Value, Map};
use sign_in::SignInModel;
use update_post::UpdatePostModel;
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew_router::{prelude::*, Switch};

#[derive(cynic::Scalar, Deserialize)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(cynic::Scalar, Deserialize)]
pub struct Jsonobject(serde_json::value::Value);

mod components;
mod index;
mod initial;
mod new_post;
mod post;
mod sign_in;
mod update_post;

pub mod query_dsl {
    use crate::{DateTime, Jsonobject};

    cynic::query_dsl!("schema.graphql");
}

#[derive(cynic::QueryFragment, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "QueryRoot"
)]
#[serde(rename_all = "camelCase")]
pub struct SettingsConnection {
    settings: Jsonobject,
}

pub struct RootModel {
    settings: Map<String, Value>,
    fetch_target: Option<FetchTask>,
}

pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<SettingsConnection>, anyhow::Error>),
}

impl Component for RootModel {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let operation = SettingsConnection::build(());

        let query = serde_json::to_string(&operation).unwrap();

        let request = Request::post("/graphql")
            .header("Content-Type", "application/json")
            .body(Ok(query))
            .expect("Failed to build request.");
        let callback = link.callback(
            |response: Response<
                Json<Result<GraphQLResponse<SettingsConnection>, anyhow::Error>>,
            >| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );
        let target = FetchService::fetch(request, callback).expect("failed to start request");

        Self {
            settings: Map::new(),
            fetch_target: Some(target),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(graphql_response) => match graphql_response.data {
                        Some(data) => {
                            self.settings = match data.settings.0.as_object() {
                                Some(map) => map.clone(),
                                None => Map::new(),
                            };
                        }
                        None => {}
                    },
                    Err(error) => ConsoleService::info(&format!("Error: {}", error.to_string())),
                };
                self.fetch_target = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        if self.fetch_target.is_some() {
            return html! {"Loading..."};
        }

        match self.settings.get("title") {
            Some(_title) => html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<IndexModel />},
                        AppRoute::Post(id) => html!{<PostModel id={id} />},
                        AppRoute::UpdatePost(id) => html!{<UpdatePostModel id={id} />},
                       AppRoute::SignIn => html!{<SignInModel />},
                        AppRoute::NewPost => html!{<NewPostModel />},
                    }
                })
              />
            },
            None => html! {<InitialModel />},
        }
    }
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/posts/new"]
    NewPost,
    #[to = "/posts/update/{id}"]
    UpdatePost(i32),
    #[to = "/post/{id}"]
    Post(i32),
    #[to = "/sign/in"]
    SignIn,
    #[to = "/"]
    Index,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<RootModel>::new().mount_to_body();
}
