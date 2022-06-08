# videah.net

My personal website intended to be hosted on [videah.net](https://videah.net), written in 99% Rust

This is a rewrite of [videah.xyz](https://github.com/videah/videah.xyz) which was cobbled together
from bits and pieces and was dependent on an ancient javascript library for the terminal
functionality. This didn't just make it hard to add new features, it made it ***boring***. Javascript suckst
Let's use WebAssembly instead 🦀✨

This rewrite is intended to make adding stuff to my site both easy and fun because it's written in
a language I actually enjoy using. This was accomplished by using the Rust UI framework [Dioxus](https://github.com/dioxuslabs/dioxus)
which I just really nice to work with.