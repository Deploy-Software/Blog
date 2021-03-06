use crate::components::toolbar::ToolbarModel;
use crate::query_dsl;
use cynic::GraphQLResponse;
use cynic::QueryBuilder;
use serde::Deserialize;
use web_sys::Node;
use yew::format::Json;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::virtual_dom::VNode;

#[derive(cynic::QueryFragment, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "Post"
)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub text: String,
    pub summary: String,
    pub created_at: crate::DateTime,
}

#[derive(cynic::FragmentArguments)]
pub struct PostArguments {
    pub post_id: i32,
}

#[derive(cynic::QueryFragment, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "QueryRoot",
    argument_struct = "PostArguments"
)]
#[serde(rename_all = "camelCase")]
pub struct PostConnection {
    #[arguments(post_id = args.post_id)]
    pub post: Option<Post>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PostModelProps {
    pub id: i32,
}

pub struct PostModel {
    fetch_target: Option<FetchTask>,
    post: Option<Post>,
}

impl PostModel {
    pub fn title(&self) -> Html {
        match &self.post {
            Some(post) => {
                html! {{post.title.clone()}}
            }
            None => html! {{"Loading..."}},
        }
    }

    pub fn markdown_node(&self) -> Html {
        match &self.post {
            Some(post) => {
                let div = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("div")
                    .unwrap();

                div.set_inner_html(&markdown::to_html(&post.text));
                let node = Node::from(div);
                let vnode = VNode::VRef(node);

                html! {{vnode}}
            }
            None => html! {{""}},
        }
    }
}

pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<PostConnection>, anyhow::Error>),
}

impl Component for PostModel {
    type Message = Msg;
    type Properties = PostModelProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let operation = PostConnection::build(PostArguments { post_id: props.id });

        let query = serde_json::to_string(&operation).unwrap();

        let request = Request::post("/graphql")
            .header("Content-Type", "application/json")
            .body(Ok(query))
            .expect("Failed to build request.");
        let callback = link.callback(
            |response: Response<Json<Result<GraphQLResponse<PostConnection>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );
        let target = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            fetch_target: Some(target),
            post: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(graphql_response) => {
                        self.post = graphql_response.data.and_then(|data| data.post);
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
          <div class="min-h-screen">
            <ToolbarModel />
            <div class="relative py-16 bg-white overflow-hidden">
              <div class="hidden lg:block lg:absolute lg:inset-y-0 lg:h-full lg:w-full">
                <div class="relative h-full text-lg max-w-prose mx-auto" aria-hidden="true">
                  <svg class="absolute top-12 left-full transform translate-x-32" width="404" height="384" fill="none" viewBox="0 0 404 384">
                    <defs>
                      <pattern id="74b3fd99-0a6f-4271-bef2-e80eeafdf357" x="0" y="0" width="20" height="20" patternUnits="userSpaceOnUse">
                        <rect x="0" y="0" width="4" height="4" class="text-gray-200" fill="currentColor" />
                      </pattern>
                    </defs>
                    <rect width="404" height="384" fill="url(#74b3fd99-0a6f-4271-bef2-e80eeafdf357)" />
                  </svg>
                  <svg class="absolute top-1/2 right-full transform -translate-y-1/2 -translate-x-32" width="404" height="384" fill="none" viewBox="0 0 404 384">
                  <defs>
                    <pattern id="f210dbf6-a58d-4871-961e-36d5016a0f49" x="0" y="0" width="20" height="20" patternUnits="userSpaceOnUse">
                      <rect x="0" y="0" width="4" height="4" class="text-gray-200" fill="currentColor" />
                    </pattern>
                  </defs>
                  <rect width="404" height="384" fill="url(#f210dbf6-a58d-4871-961e-36d5016a0f49)" />
                </svg>
                <svg class="absolute bottom-12 left-full transform translate-x-32" width="404" height="384" fill="none" viewBox="0 0 404 384">
                  <defs>
                    <pattern id="d3eb07ae-5182-43e6-857d-35c643af9034" x="0" y="0" width="20" height="20" patternUnits="userSpaceOnUse">
                      <rect x="0" y="0" width="4" height="4" class="text-gray-200" fill="currentColor" />
                    </pattern>
                  </defs>
                  <rect width="404" height="384" fill="url(#d3eb07ae-5182-43e6-857d-35c643af9034)" />
                </svg>
              </div>
            </div>
            <div class="relative px-4 sm:px-6 lg:px-8">
              <div class="text-lg max-w-prose mx-auto">
                <h1>
                  <span class="block text-base text-center text-indigo-600 font-semibold tracking-wide uppercase">{"Introducing"}</span>
                  <span class="mt-2 block text-3xl text-center leading-8 font-extrabold tracking-tight text-gray-900 sm:text-4xl">{self.title()}</span>
                </h1>
              </div>
              <div class="text-lg max-w-prose mx-auto">
              <div class="mt-6 prose prose-indigo prose-lg text-gray-500 mx-auto">
              {self.markdown_node()}
              </div>
              </div>
            </div>
          </div>
        </div>
        }
    }
}
