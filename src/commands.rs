pub mod echo;
pub mod unknown;
pub mod markdown;
pub mod redirect;
pub mod html;
pub mod arviewer;
pub mod blank;

use dioxus::prelude::*;
use dyn_clonable::*;
use crate::commands::arviewer::ARCommand;
use crate::commands::blank::BlankCommand;

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

pub fn get_command_handler(cx: &ScopeState, args: Vec<&str>, command: &str) -> Box<dyn Command> {
    let mut execute_analytics_event = true;

    let eval = use_eval(&cx);
    let cmd_text = args[0].trim();

    let cmd: Box<dyn Command> = match cmd_text {
        "social" => Box::new(MarkdownCommand::new(command, include_str!("../static/text/social.md"))),
        "intro" => Box::new(HtmlCommand::new(command, include_str!("../static/text/intro.html"))),
        "echo" => Box::new(EchoCommand::new(command)),
        "3d-fursona" => Box::new(ARCommand::new(command, "videah", Some("Kurenai_Chi"))),
        // Redirect Commands
        "blog" | "bsky" | "fursona" | "mastodon" | "github" | "steam" | "ko-fi" => {
            let url = match cmd_text {
                "blog" => "https://blog.videah.net",
                "bsky" => "https://bsky.app/profile/videah.net",
                "fursona" => "https://refs.videah.net/videah/",
                "mastodon" => "https://meow.social/@videah",
                "github" => "https://github.com/videah",
                "steam" => "https://steamcommunity.com/id/videah",
                "ko-fi" => "https://ko-fi.com/videah",
                _ => unreachable!(), // This should never happen due to the pattern matching.
            };
            Box::new(RedirectCommand::new(command, url))
        }
        // Help Command
        "help" => Box::new(MarkdownCommand::new(command, include_str!("../static/text/help.md"))),
        "" => {
            execute_analytics_event = false;
            Box::new(BlankCommand::new())
        },
        _ => {
            execute_analytics_event = false;
            Box::new(UnknownCommand::new(command))
        },
    };

    // If it's a known command, we send an analytics event to Umami.
    if execute_analytics_event {
        let event = format!("umami.track('Command Executed ({cmd_text})')");
        eval(&event).unwrap();
    }

    cmd
}