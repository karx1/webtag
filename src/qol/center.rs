use sycamore::prelude::*;

#[component(inline_props)]
pub fn Center(children: Children) -> View {
    view! {
        div(class="text-align-center") {
            (children)
        }
    }
}
