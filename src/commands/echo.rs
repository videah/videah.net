use dioxus::prelude::*;
use crate::Command;

#[derive(Clone)]
pub struct EchoCommand {
    command_text: String,
    echo_text: String,
}

impl EchoCommand {
    pub fn new(command: &str) -> Self {
        EchoCommand { command_text: command.to_string(), echo_text: "".to_string() }
    }
}

impl Command for EchoCommand {
    fn execute(&mut self) {
        let mut args = self.command_text.split(' ').collect::<Vec<&str>>();
        args.remove(0);
        let echo_string = args.join(" ");
        let ansi_string = echo_string.replace("\\x1b", "\x1b");
        self.echo_text = ansi_to_html::convert_escaped(&ansi_string).unwrap();
    }

    fn render(&self) -> LazyNodes {
        let text = &self.echo_text;
        rsx! {
            div { dangerous_inner_html: "{text}" }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}