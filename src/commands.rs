pub mod echo;
pub mod unknown;
pub mod markdown;
pub mod redirect;
pub mod html;

use dioxus::prelude::*;
use dyn_clonable::*;

use crate::commands::html::HtmlCommand;
use crate::commands::markdown::MarkdownCommand;
use crate::commands::redirect::RedirectCommand;
use crate::commands::unknown::UnknownCommand;
use crate::commands::echo::EchoCommand;

#[clonable]
pub trait Command: Clone {
    fn execute(&mut self) {}
    fn render(&self) -> LazyNodes;
    fn text(&self) -> &String;
}

#[derive(Clone)]
pub struct SocialCommand {
    command_text: String
}

impl SocialCommand {
    pub fn new(command: &str) -> Self {
        SocialCommand { command_text: command.to_string() }
    }
}

impl Command for SocialCommand {

    fn render(&self) -> LazyNodes {
        rsx! {
            div{
                "Hello, World"
            }
        }
    }

    fn text(&self) -> &String {
        &self.command_text
    }
}

pub fn get_command_handler(args: Vec<&str>, command: &str) -> Box<dyn Command> {
    match args[0] {
        "social" => Box::new(SocialCommand::new(command)),
        "intro" => Box::new(HtmlCommand::new(command, include_str!("../static/text/intro.html"))),
        "echo" => Box::new(EchoCommand::new(command)),
        "gay" => Box::new(MarkdownCommand::new(command, include_str!("../static/text/social.md"))),
        "twitter" => Box::new(RedirectCommand::new(command, "https://twitter.com/videah_")),
        &_ => Box::new(UnknownCommand::new(command)),
    }
}