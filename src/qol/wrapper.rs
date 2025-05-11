use sycamore::prelude::*;

#[component(inline_props)]
pub fn Wrapper(children: Children) -> View {
    view! {
        div(class="wrapper") {
            (children)
        }
    }
}
