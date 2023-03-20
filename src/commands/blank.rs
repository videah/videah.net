use dioxus::prelude::*;
use crate::Command;

#[derive(Clone)]
pub struct BlankCommand {
    command_text: String,
}

impl BlankCommand {
    pub fn new() -> Self {
        BlankCommand { command_text: "".to_string() }
    }
}

impl Command for BlankCommand {
    fn render(&self) -> LazyNodes {
        rsx! {
            div{ "" }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}