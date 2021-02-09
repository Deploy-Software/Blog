use crate::query_dsl;
use crate::Jsonobject;
use cynic::GraphQLResponse;
use cynic::QueryBuilder;
use serde::Deserialize;
use serde_json::{value::Value, Map};
use yew::format::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;

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

pub struct ToolbarModel {
    settings: Map<String, Value>,
    fetch_target: Option<FetchTask>,
}

pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<SettingsConnection>, anyhow::Error>),
}

impl Component for ToolbarModel {
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
        false
    }

    fn view(&self) -> Html {
        html! {
          <header class="bg-white shadow-sm lg:static lg:overflow-y-visible">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              <div class="relative flex justify-between xl:grid xl:grid-cols-12 lg:gap-8">
                <div class="flex md:absolute md:left-0 md:inset-y-0 lg:static xl:col-span-2">
                  <div class="flex-shrink-0 flex items-center">
                    <a href="/">
                      <img class="block h-8 w-auto" src="https://tailwindui.com/img/logos/workflow-mark.svg?color=indigo&shade=500" alt="Workflow" />
                    </a>
                  </div>
                </div>
                <div class="min-w-0 flex-1 md:px-8 lg:px-0 xl:col-span-6">
                  <div class="flex items-center px-6 py-4 md:max-w-3xl md:mx-auto lg:max-w-none lg:mx-0 xl:px-0">
                    <p class="block w-full bg-white rounded-md py-2 pl-10 pr-3 text-lg placeholder-gray-500 focus:outline-none focus:text-gray-900 focus:placeholder-gray-400 focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" >{self.settings.get("title").and_then(|value| value.as_str()).unwrap_or("")}{" - "}{self.settings.get("tagline").and_then(|value| value.as_str()).unwrap_or("")}</p>
                  </div>
                </div>
                <div class="flex items-center md:absolute md:right-0 md:inset-y-0 lg:hidden">
                  <button type="button" class="-mx-2 rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500" aria-expanded="false">
                    <span class="sr-only">{"Open menu"}</span>
                    <svg class="block h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                    </svg>
                    <svg class="hidden h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                <div class="hidden lg:flex lg:items-center lg:justify-end xl:col-span-4">
                  <a href="/sign/in" class="ml-6 px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                    {"Sign in"}
                  </a>
                </div>
              </div>
            </div>
            <nav class="hidden lg:hidden" aria-label="Global">
              <div class="max-w-3xl mx-auto px-2 pt-2 pb-3 space-y-1 sm:px-4">
                <a href="#" aria-current="page" class="bg-gray-100 text-gray-900 block rounded-md py-2 px-3 text-base font-medium text-gray-900">{"Home"}</a>

                <a href="#" aria-current="false" class="hover:bg-gray-50 block rounded-md py-2 px-3 text-base font-medium text-gray-900">{"Popular"}</a>

                <a href="#" aria-current="false" class="hover:bg-gray-50 block rounded-md py-2 px-3 text-base font-medium text-gray-900">{"Communities"}</a>

                <a href="#" aria-current="false" class="hover:bg-gray-50 block rounded-md py-2 px-3 text-base font-medium text-gray-900">{"Trending"}</a>
              </div>
              <div class="border-t border-gray-200 pt-4 pb-3">
                <div class="max-w-3xl mx-auto px-4 flex items-center sm:px-6">
                  <div class="flex-shrink-0">
                    <img class="h-10 w-10 rounded-full" src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&amp;ixid=eyJhcHBfaWQiOjEyMDd9&amp;auto=format&amp;fit=facearea&amp;facepad=2&amp;w=256&amp;h=256&amp;q=80" alt="" />
                  </div>
                  <div class="ml-3">
                    <div class="text-base font-medium text-gray-800">{"Chelsea Hagon"}</div>
                    <div class="text-sm font-medium text-gray-500">{"chelseahagon@example.com"}</div>
                  </div>
                  <button type="button" class="ml-auto flex-shrink-0 bg-white rounded-full p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                    <span class="sr-only">{"View notifications"}</span>
                    <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
                    </svg>
                  </button>
                </div>
              </div>
            </nav>
          </header>
        }
    }
}
