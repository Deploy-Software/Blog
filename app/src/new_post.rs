use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::Node;


pub struct NewPostModel {
    link: ComponentLink<Self>,
    text: String,
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
  
          html!{{vnode}}
    }
}

pub enum Msg {
    Change(String),
}

impl Component for NewPostModel {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(text) => {
                self.text = text;
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
            <div id="markdown">
                <header>
                    <p>
                        {"Yew Markdown Preview: "}
                    </p>
                </header>
                <div class={"container"}>
                    <textarea
                        value=&self.text
                        oninput=self.link.callback(|input_data: InputData| Msg::Change(input_data.value))
                    />
                    <div class={"prose"}>{self.markdown_node()}</div>
                </div>
            </div>
        }
    }
}
