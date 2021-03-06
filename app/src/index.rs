use crate::components::toolbar::ToolbarModel;
use crate::post::Post;
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
pub struct PostsConnection {
    posts: Vec<Post>,
    settings: Jsonobject,
}

pub struct IndexModel {
    posts: Vec<Post>,
    settings: Map<String, Value>,
    fetch_target: Option<FetchTask>,
}

impl IndexModel {
    pub fn render_post(&self, post: &Post) -> Html {
        html! {
          <a href={format!("/post/{}", post.id)}>
          <li class="bg-white px-4 py-6 shadow sm:p-6 sm:rounded-lg mt-4">
              <article aria-labelledby="question-title-81614">
                <div>
                  <h2 id="question-title-81614" class="mb-4 text-base font-medium text-gray-900">
                    {&post.title}
                  </h2>
                  <div class="flex space-x-3">
                  <div class="flex-shrink-0">
                    <img class="h-10 w-10 rounded-full" src="https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-1.2.1&amp;ixid=eyJhcHBfaWQiOjEyMDd9&amp;auto=format&amp;fit=facearea&amp;facepad=2&amp;w=256&amp;h=256&amp;q=80" alt="" />
                  </div>
                  <div class="min-w-0 flex-1">
                    <p class="text-sm font-medium text-gray-900">
                      <a href="#" class="hover:underline">{"Dries Vincent"}</a>
                    </p>
                    <p class="text-sm text-gray-500">
                      <a href="#" class="hover:underline">{post.created_at.0.format("%Y-%m-%d")}</a>
                    </p>
                  </div>
                </div>
                </div>
                <div class="mt-2 text-sm text-gray-700 space-y-4">
                  <p>
                    {&post.summary}
                  </p>
                </div>
                <div class="mt-6 flex justify-between space-x-8">
                  <div class="flex space-x-6">
                    <span class="inline-flex items-center text-sm">
                      <button class="inline-flex space-x-2 text-gray-400 hover:text-gray-500">
                        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                          <path d="M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z" />
                        </svg>
                        <span class="font-medium text-gray-900">{"29"}</span>
                        <span class="sr-only">{"likes"}</span>
                      </button>
                    </span>
                    <span class="inline-flex items-center text-sm">
                      <button class="inline-flex space-x-2 text-gray-400 hover:text-gray-500">
                        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                          <path fill-rule="evenodd" d="M18 5v8a2 2 0 01-2 2h-5l-5 4v-4H4a2 2 0 01-2-2V5a2 2 0 012-2h12a2 2 0 012 2zM7 8H5v2h2V8zm2 0h2v2H9V8zm6 0h-2v2h2V8z" clip-rule="evenodd" />
                        </svg>
                        <span class="font-medium text-gray-900">{"11"}</span>
                        <span class="sr-only">{"replies"}</span>
                      </button>
                    </span>
                    <span class="inline-flex items-center text-sm">
                      <button class="inline-flex space-x-2 text-gray-400 hover:text-gray-500">
                        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                          <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                          <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
                        </svg>
                        <span class="font-medium text-gray-900">{"2.7k"}</span>
                        <span class="sr-only">{"views"}</span>
                      </button>
                    </span>
                  </div>
                </div>
              </article>
            </li>
            </a>
        }
    }
}

pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<PostsConnection>, anyhow::Error>),
}

impl Component for IndexModel {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let operation = PostsConnection::build(());

        let query = serde_json::to_string(&operation).unwrap();

        let request = Request::post("/graphql")
            .header("Content-Type", "application/json")
            .body(Ok(query))
            .expect("Failed to build request.");
        let callback = link.callback(
            |response: Response<Json<Result<GraphQLResponse<PostsConnection>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );
        let target = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            posts: vec![],
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
                            self.posts = data.posts;
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
          <div class="min-h-screen bg-gray-100">
            <ToolbarModel />
            <div class="py-10">
              <div class="max-w-3xl mx-auto sm:px-6 lg:px-8 lg:grid lg:grid-cols-12 lg:gap-8">
                <main class="col-span-12">
                  <h1 class="sr-only">{"Recent posts"}</h1>
                  <ul>
                    { for self.posts.iter().map(|post| self.render_post(post)) }
                  </ul>
                </main>
              </div>
            </div>
          </div>
        }
    }
}
