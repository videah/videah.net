mod commands;

use dioxus::prelude::*;
use crate::commands::Command;
use crate::commands::html::HtmlCommand;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(PartialEq, Props)]
struct PromptProps<'a> {
    command: &'a str,
}

fn prompt<'a>(cx: Scope<'a, PromptProps<'a>>) -> Element {
    if cx.props.command == "__super_secret_intro_hide_prompt_command" {
        cx.render(rsx! { div {} })
    } else {
        cx.render(rsx! {
        p {
            class: "hidden",
            span { class: "prompt", id: "hostname", "guest@videah.net" },
            span { class: "prompt", id: "hblock", "" },
            span { class: "prompt", id: "fs", "~" },
            span { class: "prompt", id: "fsblock", "" },
            span { class: "input", "{cx.props.command}" },
        }
    })
    }
}

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let input_value = use_state(&cx, || "".to_string());

    let intro: Box<dyn Command> = Box::new(
        HtmlCommand::new(
            "__super_secret_intro_hide_prompt_command",
            include_str!("../static/text/intro.html"),
        ),
    );
    let elements: &UseState<Vec<Box<dyn Command>>> = use_state(&cx, || vec![intro]);

    let rustc = built_info::RUSTC_VERSION;
    let onload = include_str!("../static/text/onload.html");

    cx.render(rsx! {
        div {
            id: "terminal",
            class: "fade-in zoom-in",
            div {
                // This is... really nasty. Basically as far as I can tell there is no way to
                // execute code AFTER a render currently. This leads to problems because we need
                // to be able to scroll new prompts into view whenever a command is run.
                //
                // So for now we have to have a MutationObserver running on the javascript side to
                // automatically scroll/focus after we're done re-rendering. But to do that we need
                // to start observing AFTER a render.. which we can't do!
                //
                // So instead we use an img tag with an empty src. This lets us run code once the
                // terminal element is mounted using the tags onerror attribute. This is so bad
                // that I'm writing this big comment to give me the motivation to fix it in time.
                // TODO: Replace this with pure Rust code, preferably one that isn't a complete hack
                dangerous_inner_html: "{onload}",
            },
            div {
                id: "elements",
                elements.iter().map(|elm| rsx! { crate::prompt { command: elm.text() }, elm.render() }),
            },
            p {
                class: "hidden",
                span { class: "prompt", id: "hostname", "guest@videah.net" },
                span { class: "prompt", id: "hblock", "" },
                span { class: "prompt", id: "fs", "~" },
                span { class: "prompt", id: "fsblock", "" },
                span {
                    id: "input-span",
                    input {
                        class: "input",
                        id: "current-input",
                        oninput: move |evt| input_value.set(evt.value.clone()),
                        prevent_default: "oninput",
                        value: "{input_value}",
                        onkeydown: move |evt| {
                            if evt.key == "Enter" {
                                let command = input_value.get();
                                let args = command.split(" ").collect::<Vec<&str>>();
                                if args[0] == "clear" {
                                    elements.make_mut().clear();
                                } else {
                                    let mut command = commands::get_command_handler(args, command);
                                    command.execute();
                                    elements.make_mut().push(command);
                                }
                                input_value.set("".to_string());

                            }
                        },
                        "{input_value}"
                    },
                }
            }
        },
        footer {
            class: "fade-in",
            p { "Built with {rustc}" }
        }
    })
}