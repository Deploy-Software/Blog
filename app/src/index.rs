use crate::query_dsl;
use cynic::GraphQLResponse;
use cynic::QueryBuilder;
use serde::Deserialize;
use yew::format::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;

#[derive(cynic::QueryFragment, Debug, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "QueryRoot"
)]
#[serde(rename_all = "camelCase")]
pub struct PingConnection {
    ping: String,
}

pub struct IndexModel {
    link: ComponentLink<Self>,
    fetch_target: Option<FetchTask>,
    value: i64,
}

pub enum Msg {
    AddOne,
    ReceiveResponse(Result<GraphQLResponse<PingConnection>, anyhow::Error>),
}

impl Component for IndexModel {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_target: None,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                let operation = PingConnection::build(());

                let query = serde_json::to_string(&operation).unwrap();

                let request = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(Ok(query))
                    .expect("Failed to build request.");
                let callback = self.link.callback(
                    |response: Response<
                        Json<Result<GraphQLResponse<PingConnection>, anyhow::Error>>,
                    >| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let target =
                    FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_target = Some(target);

                self.value += 1
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(graphql_response) => {
                        ConsoleService::info(&format!("OK: {:?}", graphql_response.data))
                    }
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
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
