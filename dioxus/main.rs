// cargo install dioxus-cli
// dx --version
// dx new hello
// dx serve --port 3000

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx!("Hello World")
}
