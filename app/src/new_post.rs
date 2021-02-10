use crate::components::toolbar::ToolbarModel;
use crate::post::Post;
use crate::query_dsl;
use crate::AppRoute;
use cynic::GraphQLResponse;
use cynic::MutationBuilder;
use serde::Deserialize;
use web_sys::Node;
use yew::virtual_dom::VNode;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
    services::storage::{Area, StorageService},
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

#[derive(cynic::FragmentArguments)]
pub struct NewPostArguments {
    title: String,
    text: String,
    slug: String,
    summary: String,
}

#[derive(cynic::QueryFragment, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "MutationRoot",
    argument_struct = "NewPostArguments"
)]
#[serde(rename_all = "camelCase")]
pub struct NewPostConnection {
    #[arguments(title = args.title.clone(), text = args.text.clone(), slug = args.slug.clone(), summary = args.summary.clone())]
    new_post: Post,
}

pub struct NewPostModel {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    slug: String,
    slug_error: Option<String>,
    summary: String,
    summary_error: Option<String>,
    title: String,
    title_error: Option<String>,
    text: String,
    text_error: Option<String>,
    error: Option<String>,
    success: Option<String>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

impl NewPostModel {
    pub fn markdown_node(&self) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        div.set_inner_html(&markdown::to_html(&self.text));
        let node = Node::from(div);
        let vnode = VNode::VRef(node);

        html! {{vnode}}
    }
    fn view_success(&self) -> Html {
        if let Some(ref _success) = self.success {
            html! {
                          <div class="fixed inset-0 flex items-end justify-center px-4 py-6 pointer-events-none sm:p-6 sm:items-start sm:justify-end">

                <div class="max-w-sm w-full bg-white shadow-lg rounded-lg pointer-events-auto ring-1 ring-black ring-opacity-5 overflow-hidden">
                  <div class="p-4">
                    <div class="flex items-start">
                      <div class="flex-shrink-0">
                        <svg class="h-6 w-6 text-green-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                      </div>
                      <div class="ml-3 w-0 flex-1 pt-0.5">
                        <p class="text-sm font-medium text-gray-900">
                        {"You are now logged in"}
                        </p>
                        <p class="mt-1 text-sm text-gray-500">
                        {"You will be redirected in a few seconds"}
                        </p>
                      </div>
                      <div class="ml-4 flex-shrink-0 flex">
                        <button class="bg-white rounded-md inline-flex text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" onclick=self.link.callback(|_| Msg::ClearNotifications)>
                          <span class="sr-only">{"Close"}</span>
                          <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                          </svg>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            }
        } else {
            html! {}
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! {
              <div class="fixed inset-0 flex items-end justify-center px-4 py-6 pointer-events-none sm:p-6 sm:items-start sm:justify-end">
                <div class="max-w-sm w-full bg-white shadow-lg rounded-lg pointer-events-auto ring-1 ring-black ring-opacity-5 overflow-hidden">
                  <div class="p-4">
                    <div class="flex items-start">
                      <div class="flex-shrink-0">
                        <svg class="h-6 w-6 text-green-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                      </div>
                      <div class="ml-3 w-0 flex-1 pt-0.5">
                        <p class="text-sm font-medium text-gray-900">
                        {"An error has occured"}
                        </p>
                        <p class="mt-1 text-sm text-gray-500">
                        {error.clone()}
                        </p>
                      </div>
                      <div class="ml-4 flex-shrink-0 flex">
                        <button class="bg-white rounded-md inline-flex text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" onclick=self.link.callback(|_| Msg::ClearNotifications)>
                          <span class="sr-only">{"Close"}</span>
                          <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                          </svg>
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            }
        } else {
            html! {}
        }
    }
}

pub enum Msg {
    SubmitNewPost,
    ChangeTitle(String),
    ChangeSlug(String),
    ChangeSummary(String),
    ChangeText(String),
    ReceiveResponse(Result<GraphQLResponse<NewPostConnection>, anyhow::Error>),
    ClearNotifications,
    Ignore,
}

impl Component for NewPostModel {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            slug: String::from(""),
            slug_error: None,
            summary: String::from(""),
            summary_error: None,
            title: String::from(""),
            title_error: None,
            text: String::from(""),
            text_error: None,
            error: None,
            success: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SubmitNewPost => {
                let storage = StorageService::new(Area::Local).unwrap();

                let token: String = match storage.restore("auth_token") {
                    Ok(token) => token,
                    Err(_err) => {
                        self.error = Some("Your authorization token is not valid".into());
                        return true;
                    }
                };

                if self.text.is_empty() {
                    self.text_error = Some("Your text is not valid".into());
                    return true;
                }

                let operation = NewPostConnection::build(NewPostArguments {
                    title: self.title.clone(),
                    text: self.text.clone(),
                    slug: self.slug.clone(),
                    summary: self.summary.clone(),
                });

                let query = serde_json::to_string(&operation).unwrap();

                let request = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .header("token", token)
                    .body(Ok(query))
                    .expect("Failed to build request.");
                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<
                        Json<Result<GraphQLResponse<NewPostConnection>, anyhow::Error>>,
                    >| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
            }
            Msg::ChangeSlug(slug) => {
                self.slug = slug;
                self.slug_error = None;
            }
            Msg::ChangeSummary(summary) => {
                self.summary = summary;
                self.summary_error = None;
            }
            Msg::ChangeTitle(title) => {
                self.title = title;
                self.title_error = None;
            }
            Msg::ChangeText(text) => {
                self.text = text;
                self.text_error = None;
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(graphql_response) => {
                        if graphql_response.errors.is_some() {
                            self.error = Some(
                                graphql_response
                                    .errors
                                    .unwrap()
                                    .into_iter()
                                    .map(|error| error.message)
                                    .collect(),
                            );
                        }
                        if graphql_response.data.is_some() {
                            let post = graphql_response.data.unwrap().new_post;
                            self.success = Some("OK".into());
                            self.router_agent
                                .send(ChangeRoute(AppRoute::Post(post.id).into()));
                            self.success = Some("OK".into());
                            self.text = String::from("");
                            self.title = String::from("");
                            self.slug = String::from("");
                            self.summary = String::from("");
                        }
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
            }
            Msg::Ignore => return false,
            Msg::ClearNotifications => {
                self.error = None;
                self.success = None;
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
            <ToolbarModel />
            <div class="bg-white lg:static lg:overflow-y-visible">
              <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="space-y-8 divide-y divide-gray-200">
                  <div class="space-y-8 divide-y divide-gray-200">
                    <div>
                      <div>
                        <h3 class="text-lg leading-6 font-medium text-gray-900">
                          {"New Post"}
                        </h3>
                        <p class="mt-1 text-sm text-gray-500">
                          {"Write a new blog post below."}
                        </p>
                      </div>

                      <div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                      <div class="sm:col-span-6">
                      <label for="username" class="block text-sm font-medium text-gray-700">
                        {"Slug"}
                      </label>
                      <div class="mt-1 flex rounded-md">
                        <span class="inline-flex items-center px-3 rounded-l-md border border-r-0 border-gray-300 bg-gray-50 text-gray-500 sm:text-sm">
                          {"/post/"}
                        </span>
                        <input
                        value=&self.slug
                        oninput=self.link.callback(|input_data: InputData| Msg::ChangeSlug(input_data.value))
                        type="text" name="username" id="username" autocomplete="username" class="flex-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full min-w-0 rounded-none rounded-r-md sm:text-sm border-gray-300" />
                      </div>
                      </div>
                      <div class="sm:col-span-6">
                        <label for="about" class="block text-sm font-medium text-gray-700">
                          {"Summary"}
                        </label>
                        <div class="mt-1">
                          <textarea
                          value=&self.summary
                          oninput=self.link.callback(|input_data: InputData| Msg::ChangeSummary(input_data.value))
                          id="about" name="about" rows="3" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"></textarea>
                        </div>
                      </div>
                      <div class="sm:col-span-6">
                      <label for="street_address" class="block text-sm font-medium text-gray-700">
                        {"Title"}
                      </label>
                      <div class="mt-1">
                        <input
                        value=&self.title
                        oninput=self.link.callback(|input_data: InputData| Msg::ChangeTitle(input_data.value))
                        type="text" name="street_address" id="street_address" autocomplete="street-address" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
                      </div>
                    </div>
                        </div>
                        </div>
                        <div class="bg-white lg:static lg:overflow-y-visible">
                        <label for="about" class="block text-sm font-medium text-gray-700">
                        {"Text"}
                      </label>
                      <div class="mt-1">
                        <div id="markdown">
                            <div class={"container"}>
                              <textarea
                              value=&self.text
                              oninput=self.link.callback(|input_data: InputData| Msg::ChangeText(input_data.value))
                              />
                              <div class={"prose"}>{self.markdown_node()}</div>
                            </div>
                          </div>
                          </div>
                      </div>
                  </div>

                  <div class="pt-5">
                    <div class="flex justify-end">
                      <button type="button" class="bg-white py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                        {"Cancel"}
                      </button>
                      <button onclick=self.link.callback(|_| Msg::SubmitNewPost) class="ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                        {"Submit New Post"}
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            { self.view_success() }
            { self.view_error() }
          </div>
        }
    }
}
