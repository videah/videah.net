use dioxus::prelude::*;
use crate::Command;

#[derive(Clone)]
pub struct HtmlCommand {
    command_text: String,
    html: &'static str,
}

impl HtmlCommand {
    pub fn new(command: &str, html: &'static str) -> Self {
        HtmlCommand { command_text: command.to_string(), html }
    }
}

impl Command for HtmlCommand {
    fn render(&self) -> LazyNodes {
        let text = self.html;
        rsx! {
            div { dangerous_inner_html: "{text}" }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}