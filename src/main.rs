use qol::{Center, Wrapper};
use sycamore::prelude::*;

mod qol;

fn main() {
    sycamore::render(|| {
        view! {
            Wrapper {
                Center {
                    h1 { "WebTag" }
                    p { "Hello, world!" }
                }
            }
        }
    });
}
