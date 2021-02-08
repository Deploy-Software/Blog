use crate::query_dsl;
use cynic::GraphQLResponse;
use cynic::MutationBuilder;
use serde::Deserialize;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(cynic::FragmentArguments)]
pub struct SignUpArguments {
    email: String,
    name: String,
    password: String,
}

#[derive(cynic::QueryFragment, Debug, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "MutationRoot",
    argument_struct = "SignUpArguments"
)]
#[serde(rename_all = "camelCase")]
pub struct SignUpConnection {
    #[arguments(email = args.email.clone(), name = args.name.clone(), password = args.password.clone())]
    sign_up: String,
}

#[derive(Debug)]
pub enum Msg {
    SignUp,
    ReceiveResponse(Result<GraphQLResponse<SignUpConnection>, anyhow::Error>),
    BlogInputReceived(String),
    EmailInputReceived(String),
    NameInputReceived(String),
    PasswordInputReceived(String),
    ClearNotifications,
}

#[derive(Debug)]
pub struct InitialModel {
    fetch_task: Option<FetchTask>,
    iss: Option<SignUpConnection>,
    link: ComponentLink<Self>,
    email: String,
    email_error: Option<String>,
    name: String,
    name_error: Option<String>,
    password: String,
    password_error: Option<String>,
    blog: String,
    blog_error: Option<String>,
    error: Option<String>,
    success: Option<String>,
}
/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl InitialModel {
    fn view_iss_location(&self) -> Html {
        match self.iss {
            Some(ref space_station) => {
                html! {
                    <>
                        <p>{ "The ISS is at:" }</p>
                        <p>{ format!("Latitude: {}", space_station.sign_up) }</p>
                    </>
                }
            }
            None => {
                html! {
                   <button
                     class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                     onclick=self.link.callback(|_| Msg::SignUp)
                   >
                    { self.view_fetching() }
                   </button>
                }
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { { "Signing up..." } }
        } else {
            html! { { "Sign up" } }
        }
    }

    fn view_blog_error(&self) -> Html {
        if let Some(ref message) = self.blog_error {
            html! {
                <p class="mt-1 text-red-500 text-sm">{ message }</p>
            }
        } else {
            html! {}
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

    fn view_name_error(&self) -> Html {
      if let Some(ref message) = self.name_error {
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
                        {"Your account has been created"}
                        </p>
                        <p class="mt-1 text-sm text-gray-500">
                        {"Check your email"}
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
impl Component for InitialModel {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            iss: None,
            link,
            error: None,
            success: None,
            email: String::from(""),
            email_error: None,
            name: String::from(""),
            name_error: None,
            password: String::from(""),
            password_error: None,
            blog: String::from(""),
            blog_error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            SignUp => {
                if self.blog.is_empty() {
                    self.blog_error = Some("Your blog name is not valid".into());
                    return true;
                }

                if self.email.is_empty() {
                    self.email_error = Some("Your email is not valid".into());
                    return true;
                }

                if self.name.is_empty() {
                  self.name_error = Some("Your name is not valid".into());
                  return true;
                }

                if self.password.is_empty() {
                    self.password_error = Some("Your password is not valid".into());
                    return true;
                }

                let operation = SignUpConnection::build(SignUpArguments {
                    email: self.email.clone(),
                    name: self.name.clone(),
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
                        Json<Result<GraphQLResponse<SignUpConnection>, anyhow::Error>>,
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
                            self.success = Some("OK".into());
                            self.email = String::from("");
                            self.name = String::from("");
                            self.password = String::from("");
                            self.blog = String::from("");
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
            BlogInputReceived(value) => {
                self.error = None;
                self.success = None;
                self.blog_error = None;
                self.blog = value;
                true
            }
            EmailInputReceived(value) => {
                self.error = None;
                self.success = None;
                self.email_error = None;
                self.email = value;
                true
            }
            NameInputReceived(value) => {
              self.error = None;
              self.success = None;
              self.name_error = None;
              self.name = value;
              true
          }
            PasswordInputReceived(value) => {
                self.error = None;
                self.success = None;
                self.password_error = None;
                self.password = value;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
              <>
        <div class="min-h-screen bg-gray-50 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
          <div class="sm:mx-auto sm:w-full sm:max-w-md">
            <img
              class="mx-auto h-12 w-auto"
              src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
              alt="Workflow"
            />
            <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
            {"Create your blog"}
            </h2>
          </div>

          <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
            <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
              <div class="space-y-6">
                 <div>
                  <label
                    htmlFor="blog"
                    class="block text-sm font-medium text-gray-700"
                  >
                  {"Blog name"}
                  </label>
                  <div class="mt-1">
                    <input
                      id="blog"
                      name="blog"
                      type="text"
                      class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                      placeholder="Ford Motor Company"
                      oninput=self.link.callback(|input_data: InputData| Msg::BlogInputReceived(input_data.value))
                        value=&self.blog
                      />
                  </div>
                  { self.view_blog_error() }
                </div>

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
                      oninput=self.link.callback(|input_data: InputData| Msg::EmailInputReceived(input_data.value))
                      value=&self.email
                      placeholder="henry@ford.com"
                    />
                    { self.view_email_error() }
                  </div>
                </div>

                <div>
                <label
                  htmlFor="name"
                  class="block text-sm font-medium text-gray-700"
                >
                {"Name"}
                </label>
                <div class="mt-1">
                  <input
                    id="name"
                    name="name"
                    type="text"
                    autoComplete="name"
                    class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                    oninput=self.link.callback(|input_data: InputData| Msg::NameInputReceived(input_data.value))
                    value=&self.name
                    placeholder="Henry Ford"
                  />
                  { self.view_name_error() }
                </div>
              </div>

                <div>
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
                    { self.view_password_error() }
                  </div>
                </div>

                <div>
                    { self.view_iss_location() }
                </div>
             </div>
            </div>
          </div>
        </div>

        { self.view_success() }
        { self.view_error() }
              </>
          }
    }
}