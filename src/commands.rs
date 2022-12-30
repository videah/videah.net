pub mod echo;
pub mod unknown;
pub mod markdown;
pub mod redirect;
pub mod html;
pub mod arviewer;

use dioxus::prelude::*;
use dyn_clonable::*;
use crate::commands::arviewer::ARCommand;

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

pub fn get_command_handler(args: Vec<&str>, command: &str) -> Box<dyn Command> {
    match args[0] {
        "social" => Box::new(MarkdownCommand::new(command, include_str!("../static/text/social.md"))),
        "intro" => Box::new(HtmlCommand::new(command, include_str!("../static/text/intro.html"))),
        "echo" => Box::new(EchoCommand::new(command)),
        "3d-fursona" => Box::new(ARCommand::new(command, "videah", Some("Kurenai_Chi"))),
        // Redirect Commands
        "twitter" => Box::new(RedirectCommand::new(command, "https://twitter.com/videah_")),
        "fursona" => Box::new(RedirectCommand::new(command, "https://refs.videah.net/videah/")),
        "mastodon" => Box::new(RedirectCommand::new(command, "https://meow.social/@videah")),
        "cohost" => Box::new(RedirectCommand::new(command, "https://www.cohost.org/videah")),
        "github" => Box::new(RedirectCommand::new(command, "https://github.com/videah")),
        "steam" => Box::new(RedirectCommand::new(command, "https://steamcommunity.com/id/videah")),
        "ko-fi" => Box::new(RedirectCommand::new(command, "https://ko-fi.com/videah")),
        // Help Command
        "help" => Box::new(MarkdownCommand::new(command, include_str!("../static/text/help.md"))),
        &_ => Box::new(UnknownCommand::new(command)),
    }
}