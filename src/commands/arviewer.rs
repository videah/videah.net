use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use crate::Command;

#[derive(Clone)]
pub struct ARCommand {
    command_text: String,
    model_name: String,
    credit_username: Option<String>,
    is_supported: bool,
}

impl ARCommand {
    pub fn new(command: &str, model_name: &str, credit_username: Option<&str>) -> Self {
        let credit = credit_username.map(str::to_string);
        ARCommand {
            command_text: command.to_string(),
            model_name: model_name.to_string(),
            credit_username: credit,
            is_supported: true,
        }
    }
}

impl Command for ARCommand {

    fn execute(&mut self) {
        // Feature check to make sure AR is supported on the platform the user is using.
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let a = document.create_element("a").unwrap();
        let link = a.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
        let token_list = link.rel_list();
        self.is_supported = token_list.supports("ar").unwrap();
    }

    fn render(&self) -> LazyNodes {
        let name = &self.model_name;
        let render_credit = match &self.credit_username {
            None => rsx!{ br {} },
            Some(username) => rsx!{
                p {
                    class: "credit",
                    "3D model by ",
                    a {
                        href: "https://twitter.com/{username}",
                        "@{username}"
                    }
                }
            }
        };

        if self.is_supported {
            return rsx! {
                br {},
                p { "Tap the image to view {name} in three dee!"},
                render_credit,
                a {
                    rel: "ar",
                    href: "static/models/{name}.usdz",
                    img {
                        class: "image-model",
                        src: "static/images/ar-previews/{name}.png"
                    }
                }
            }
        } else {
            return rsx! {
                p { "This command is only supported on iPhone and iPad for now :("}
            }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}