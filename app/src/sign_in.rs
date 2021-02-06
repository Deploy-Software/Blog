use crate::query_dsl;
use crate::AppRoute;
use cynic::GraphQLResponse;
use cynic::MutationBuilder;
use serde::Deserialize;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
    services::storage::{Area, StorageService},
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

#[derive(cynic::FragmentArguments)]
pub struct SignInArguments {
    email: String,
    password: String,
}

#[derive(cynic::QueryFragment, Debug, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "MutationRoot",
    argument_struct = "SignInArguments"
)]
#[serde(rename_all = "camelCase")]
pub struct SignInConnection {
    #[arguments(email = args.email.clone(), password = args.password.clone())]
    sign_in: String,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {}

#[derive(Debug)]
pub enum Msg {
    SignIn,
    ReceiveResponse(Result<GraphQLResponse<SignInConnection>, anyhow::Error>),
    EmailInputReceived(String),
    PasswordInputReceived(String),
    RememberMeInputReceived,
    ClearNotifications,
    Ignore,
}

pub struct SignInModel {
    fetch_task: Option<FetchTask>,
    sign_in_action: Option<SignInConnection>,
    link: ComponentLink<Self>,
    email: String,
    email_error: Option<String>,
    password: String,
    password_error: Option<String>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    error: Option<String>,
    success: Option<String>,
}

impl SignInModel {
    fn view_button(&self) -> Html {
        match self.sign_in_action {
            Some(ref response) => {
                html! {
                    <>
                        <p>{ &response.sign_in }</p>
                    </>
                }
            }
            None => {
                html! {
                   <button
                     class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                     onclick=self.link.callback(|_| Msg::SignIn)
                   >
                    { self.view_fetching() }
                   </button>
                }
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { { "Signing in..." } }
        } else {
            html! { { "Sign in" } }
        }
    }

    fn view_email_error(&self) -> Html {
        if let Some(ref message) = self.email_error {
            html! {
                <p class="mt-1 text-red-500 text-sm">{ message }</p>
            }
        } else {
            html! {}
        }
    }

    fn view_password_error(&self) -> Html {
        if let Some(ref message) = self.password_error {
            html! {
                <p class="mt-1 text-red-500 text-sm">{ message }</p>
            }
        } else {
            html! {}
        }
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

/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl Component for SignInModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            sign_in_action: None,
            error: None,
            success: None,
            email: String::from(""),
            email_error: None,
            password: String::from(""),
            password_error: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            SignIn => {
                if self.email.is_empty() {
                    self.email_error = Some("Your email is not valid".into());
                    return true;
                }

                if self.password.is_empty() {
                    self.password_error = Some("Your password is not valid".into());
                    return true;
                }

                let operation = SignInConnection::build(SignInArguments {
                    email: self.email.clone(),
                    password: self.password.clone(),
                });

                let query = serde_json::to_string(&operation).unwrap();

                let request = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(Ok(query))
                    .expect("Failed to build request.");
                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<
                        Json<Result<GraphQLResponse<SignInConnection>, anyhow::Error>>,
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
                true
            }
            ReceiveResponse(response) => {
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
                            let token = graphql_response.data.unwrap().sign_in;
                            let mut storage = StorageService::new(Area::Local).unwrap();
                            storage.store("auth_token", Ok(token.clone()));
                            self.success = Some("OK".into());
                            self.router_agent
                                .send(ChangeRoute(AppRoute::Index.into()));
                            self.email = String::from("");
                            self.password = String::from("");
                        }
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
            ClearNotifications => {
                self.error = None;
                self.success = None;
                true
            }
            EmailInputReceived(value) => {
                self.error = None;
                self.success = None;
                self.email_error = None;
                self.email = value;
                true
            }
            PasswordInputReceived(value) => {
                self.error = None;
                self.success = None;
                self.password_error = None;
                self.password = value;
                true
            }
            RememberMeInputReceived => {
                self.error = None;
                self.success = None;
                true
            }
            Ignore => false,
        }
    }
    fn view(&self) -> Html {
        html! {
          <div class="min-h-screen bg-white flex">
          <div class="flex-1 flex flex-col justify-center py-12 px-4 sm:px-6 lg:flex-none lg:px-20 xl:px-24">
            <div class="mx-auto w-full max-w-sm lg:w-96">
              <div>
                <img
                  class="h-12 w-auto"
                  src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
                  alt="Workflow"
                />
                <h2 class="mt-6 text-3xl font-extrabold text-gray-900">
                  {"Sign in to your account"}
                </h2>
             </div>

              <div class="mt-8">
                <div>
                  <div class="space-y-6">
                    <div>
                      <label
                        htmlFor="email"
                        class="block text-sm font-medium text-gray-700"
                      >
                        {"Email address"}
                      </label>
                      <div class="mt-1">
                        <input
                          id="email"
                          name="email"
                          type="email"
                          autoComplete="email"
                          class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                          placeholder="henry@ford.com"
                          oninput=self.link.callback(|input_data: InputData| Msg::EmailInputReceived(input_data.value))
                          value=&self.email
                        />
                      </div>
                      { self.view_email_error() }
                    </div>

                    <div class="space-y-1">
                      <label
                        htmlFor="password"
                        class="block text-sm font-medium text-gray-700"
                      >
                        {"Password"}
                      </label>
                      <div class="mt-1">
                        <input
                          id="password"
                          name="password"
                          type="password"
                          autoComplete="current-password"
                          class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                          placeholder="*************"
                          oninput=self.link.callback(|input_data: InputData| Msg::PasswordInputReceived(input_data.value))
                          value=&self.password
                        />
                      </div>
                    </div>

                    <div class="flex items-center justify-between">
                      <div class="flex items-center">
                        <input
                          id="remember_me"
                          name="remember_me"
                          type="checkbox"
                          class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
                          oninput=self.link.callback(|_| Msg::RememberMeInputReceived)
                        />
                        <label
                          htmlFor="remember_me"
                          class="ml-2 block text-sm text-gray-900"
                        >
                          {"Remember me"}
                        </label>
                        { self.view_password_error() }
                      </div>

                      <div class="text-sm">
                        <a
                          href="/sign/up"
                          class="font-medium text-indigo-600 hover:text-indigo-500"
                        >
                          {"Don't have an account?"}
                        </a>
                      </div>
                    </div>

                    <div>
                      { self.view_button() }
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div class="hidden lg:block relative w-0 flex-1">
            <img
              class="absolute inset-0 h-full w-full object-cover"
              src="https://images.unsplash.com/photo-1505904267569-f02eaeb45a4c?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1908&q=80"
              alt=""
            />
          </div>
          { self.view_success() }
          { self.view_error() }
        </div>
         }
    }
}
