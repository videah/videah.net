use dioxus::prelude::*;
use crate::Command;

#[derive(Clone)]
pub struct MarkdownCommand {
    command_text: String,
    markdown: &'static str,
    html: String,
}

impl MarkdownCommand {
    pub fn new(command: &str, markdown: &'static str) -> Self {
        MarkdownCommand { command_text: command.to_string(), markdown, html: "".to_string() }
    }
}

impl Command for MarkdownCommand {
    fn execute(&mut self) {
        self.html = markdown::to_html(&self.markdown);
    }

    fn render(&self) -> LazyNodes {
        let text = &self.html;
        rsx! {
            // This is only dangerous from a XSS standpoint, which we don't need to worry about.
            div { dangerous_inner_html: "{text}" }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}