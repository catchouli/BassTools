use sycamore::prelude::*;

#[component]
pub fn Menu<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="column is-narrow") {
            aside(class="menu") {
                p(class="menu-label") { "Knowledge" }
                ul(class="menu-list") {
                    li { a { "Key signatures" } }
                    li { a { "Chord naming" } }
                    li { a { "Chord function" } }
                }
            }
        }
    }
}
