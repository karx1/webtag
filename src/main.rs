use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            h1(style="text-align:center") { "WebTag" }
            p(style="text-align:center") { "Hello, world!" }
        }
    });
}
