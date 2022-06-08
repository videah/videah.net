use dioxus::prelude::*;
use crate::Command;

#[derive(Clone)]
pub struct UnknownCommand {
    command_text: String,
}

impl UnknownCommand {
    pub fn new(command: &str) -> Self {
        UnknownCommand { command_text: command.to_string() }
    }
}

impl Command for UnknownCommand {

    fn render(&self) -> LazyNodes {
        let text = &self.command_text;
        rsx! {
            div{ "bash: {text}: command not found" }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}