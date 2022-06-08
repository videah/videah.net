use dioxus::prelude::*;
use crate::Command;

#[derive(Clone)]
pub struct RedirectCommand {
    command_text: String,
    redirect_url: String,
}

impl RedirectCommand {
    pub fn new(command: &str, url: &str) -> Self {
        RedirectCommand { command_text: command.to_string(), redirect_url: url.to_string() }
    }
}

impl Command for RedirectCommand {

    fn execute(&mut self) {
        web_sys::window().unwrap().location().set_href(&self.redirect_url).expect("can't redirect");
    }

    fn render(&self) -> LazyNodes {
        let text = &self.command_text;
        rsx! {
            div{ "Redirecting to {text}..." }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}